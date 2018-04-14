module Main exposing (..)

import WebSocket
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Json.Decode
import Svg
import Svg.Attributes
import List.Extra

import Graph

import Types exposing (Message(..), Reading, messageDecoder, readingsToChannels)

url : String
url = "ws://localhost:8765"


type TriggerMode
    = Continuous
    | FallingEdge
    | RisingEdge




-- Model and init

type alias Model =
    { readings: List Reading
    , currentReading: Reading
    , triggerMode: TriggerMode
    }


init : (Model, Cmd Msg)
init =
    ( { readings = []
      , currentReading = (Reading [] 0)
      , triggerMode = Continuous
    }
    , Cmd.none
    )





-- Messages

type Msg
    = NewMessage String
    | Send
    | TriggerModeSet TriggerMode





-- Update function

update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
    case msg of
        Send ->
            (model, WebSocket.send url "")
        NewMessage message ->
            let
                decoded = Json.Decode.decodeString messageDecoder message
            in
                case decoded of
                    Ok (NewReading reading) ->
                        ({model
                            | readings = model.readings ++ [reading]
                            , currentReading = reading
                        }, Cmd.none)
                    Ok (CurrentTime time) ->
                        let
                            oldReading = model.currentReading
                            newReading = {oldReading | time = time}
                        in
                            ({model | currentReading = newReading}, Cmd.none)
                    Err e ->
                        let
                            _ = Debug.log "Error decoding message: " e
                        in
                            (model, Cmd.none)
        TriggerModeSet mode ->
            ({model | triggerMode = mode}, Cmd.none)



-- Subscriptions

subscriptions : Model -> Sub Msg
subscriptions model =
    WebSocket.listen url NewMessage


stepPreprocessor : List (Float, Bool) -> List (Float, Bool)
stepPreprocessor original =
    let
        duplicated = List.Extra.interweave original original

        (times, values) = List.unzip duplicated

        shiftedTimes = List.drop 1 times
    in
        List.Extra.zip shiftedTimes values

-- View

view : Model -> Html Msg
view model =
    let
        readings = List.map stepPreprocessor 
            <| readingsToChannels (model.readings ++ [model.currentReading])

        (viewWidth, viewHeight) = (600, 50)

        graphFunction : List (Float, Bool) -> Html Msg
        graphFunction readingList =
            Svg.svg
              [ Svg.Attributes.viewBox <| "0 0 " ++ (toString viewWidth) ++ " " ++ (toString viewHeight)
              , Svg.Attributes.width <| toString viewWidth ++ "px"
              , Svg.Attributes.height <| toString viewHeight ++ "px"
              ]
              [ Graph.drawHorizontalLines (viewWidth, viewHeight) (0,1) 1
              , Graph.drawGraph (viewWidth, viewHeight) (0,1)
                <| List.map (\(time, val) -> if val then (time, 1) else (time, 0)) readingList
              ]

        currTriggerModeSymbol = case model.triggerMode of
            Continuous -> "→"
            FallingEdge -> "⤵"
            RisingEdge -> "⤴"

        buttonRow = div []
            [ label [] [text ("Trigger mode (" ++ currTriggerModeSymbol ++ "):")]
            , button [onClick (TriggerModeSet Continuous)] [text "→"]
            , button [onClick (TriggerModeSet FallingEdge)] [text "⤵"]
            , button [onClick (TriggerModeSet RisingEdge)] [text "⤴"]
            ]

        timeSpanSelection = div [] [label [] [text "Time range"], input [] []]
    in
        div []
            <|  [ buttonRow
                , timeSpanSelection
                ]
                ++
                (List.map graphFunction readings)



main =
    Html.program
        { init = init
        , view = view
        , update = update
        , subscriptions = subscriptions
        }
