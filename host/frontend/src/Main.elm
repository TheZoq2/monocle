module Main exposing (..)

import WebSocket
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Json.Decode

import Types exposing (Reading, readingDecoder)

url : String
url = "ws://localhost:8765"




-- Model and init

type alias Model =
    { readings: List Reading
    }


init : (Model, Cmd Msg)
init =
    ( { readings = [] }
    , Cmd.none
    )





-- Messages

type Msg
    = NewMessage String
    | Send





-- Update function

update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
    case msg of
        Send ->
            (model, WebSocket.send url "")
        NewMessage message ->
            let
                _ = Debug.log "message" message
                decoded = Json.Decode.decodeString readingDecoder message
            in
                case decoded of
                    Ok reading ->
                        ({model | readings = model.readings ++ [reading]}, Cmd.none)
                    Err e ->
                        let
                            _ = Debug.log "Error decoding message: " e
                        in
                            (model, Cmd.none)



-- Subscriptions

subscriptions : Model -> Sub Msg
subscriptions model = 
    WebSocket.listen url NewMessage


-- View

view : Model -> Html Msg
view model =
    div []
        [ button [onClick Send] [text "send thing"]
        , p [] [text <| toString (List.length model.readings) ]
        ]



main =
    Html.program
        { init = init
        , view = view
        , update = update
        , subscriptions = subscriptions
        }
