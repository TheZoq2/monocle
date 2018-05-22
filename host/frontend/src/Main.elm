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

import Types exposing
    ( Message(..),
    Reading,
    messageDecoder,
    readingsToChannels,
    TriggerMode(..),
    triggerModeSymbol,
    allTriggerModes
    )
import Signal exposing (edgeTrigger, isRisingEdge, isFallingEdge, continuousRead)
import TimeUnits exposing (Time, TimeUnit(..), timeUnitString, toMicroseconds, allTimeUnits)


url : String
url = "ws://localhost:8765"




-- Model and init

type alias Model =
    { readings: List Reading
    , currentReading: Reading
    , triggerMode: TriggerMode
    , timeSpan: Time
    , triggerChannel: Int
    }


init : (Model, Cmd Msg)
init =
    ( { readings = []
      , currentReading = (Reading [] 0)
      , triggerMode = FallingEdge
      , timeSpan = Time Millisecond 1
      , triggerChannel = 1
    }
    , Cmd.none
    )





-- Messages

type Msg
    = NewMessage String
    | Send
    | TriggerModeSet TriggerMode
    | TimeSpanSet String
    | TimeSpanUnitSet TimeUnit
    | TriggerChannelSet Int
    | ResetValues





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
        TimeSpanSet val ->
            let
                asFloat = Result.withDefault model.timeSpan.value <| String.toFloat val
                oldSpan = model.timeSpan
            in
                ({model | timeSpan = { oldSpan | value = asFloat }}, Cmd.none)
        TimeSpanUnitSet unit ->
            let
                oldSpan = model.timeSpan
            in
                ({model | timeSpan = { oldSpan | unit = unit }}, Cmd.none)
        TriggerChannelSet index ->
            ({model | triggerChannel = index}, Cmd.none)
        ResetValues ->
            ({model | readings = []}, Cmd.none)




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


trigFunction : TriggerMode -> (Bool -> Bool -> Bool)
trigFunction mode =
    case mode of
        Continuous -> continuousRead
        RisingEdge -> isRisingEdge
        FallingEdge -> isFallingEdge


-- View

view : Model -> Html Msg
view model =
    let
        readings = List.map stepPreprocessor 
            <| readingsToChannels (model.readings ++ [model.currentReading])

        valueRange = edgeTrigger
            (trigFunction model.triggerMode)
            (toMicroseconds model.timeSpan)
            (Maybe.withDefault [] <| List.Extra.getAt model.triggerChannel readings)

        (viewWidth, viewHeight) = (600, 50)

        graphFunction : List (Float, Bool) -> Html Msg
        graphFunction readingList =
            Svg.svg
              [ Svg.Attributes.viewBox <| "0 0 " ++ (toString viewWidth) ++ " " ++ (toString viewHeight)
              , Svg.Attributes.width <| toString viewWidth ++ "px"
              , Svg.Attributes.height <| toString viewHeight ++ "px"
              ]
              [ Graph.drawHorizontalLines (viewWidth, viewHeight) (0,1) 1
              , Graph.drawGraph (viewWidth, viewHeight) (0,1) valueRange
                <| List.map (\(time, val) -> if val then (time, 1) else (time, 0)) readingList
              ]

        triggerModeButtons = 
                singleChoiseSelector
                    model.triggerMode
                    allTriggerModes
                    triggerModeSymbol
                    TriggerModeSet

        triggerModeRow = div []
            (  [label [] [text ("Trigger mode: ")]]
            ++ triggerModeButtons
            )

        timeSpanSelection =
            div
                []
                ([ label [] [text "Time range: "]
                , input [onInput TimeSpanSet, placeholder (toString model.timeSpan.value)] []
                ]
                ++ timeUnitSelector model.timeSpan.unit TimeSpanUnitSet
                )

        triggerChannelSelector =
            div []
                ( [label [] [text "TriggerChannel: "]]
                ++ ( singleChoiseSelector
                    model.triggerChannel
                    (List.range 0 <| (List.length readings) - 1)
                    toString
                    TriggerChannelSet
                  )
                )

        buttonRow = [div [] [button [onClick ResetValues] [text "Reset"]]]
    in
        div []
            <|  [ triggerModeRow
                , triggerChannelSelector
                , timeSpanSelection
                ]
                ++
                (List.map (\reading -> div [] [graphFunction reading]) readings)
                ++
                buttonRow

timeUnitSelector : TimeUnit -> (TimeUnit -> Msg) -> List (Html Msg)
timeUnitSelector currentUnit msg =
    singleChoiseSelector currentUnit allTimeUnits timeUnitString msg


singleChoiseSelector : a -> List a -> (a -> String) -> (a -> Msg) -> List (Html Msg)
singleChoiseSelector current choises nameFunction msg =
    List.map
        (\alternative ->
            button [onClick (msg alternative)] 
                [(if alternative == current then
                    b []
                else 
                    span []
                ) [text <| nameFunction alternative]
                ]
        )
        choises



main =
    Html.program
        { init = init
        , view = view
        , update = update
        , subscriptions = subscriptions
        }
