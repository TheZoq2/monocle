module Main exposing (..)

import WebSocket
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)

url : String
url = "ws://localhost:8765"

type alias Reading =
    { time: Int
    , value: Int
    }




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
            in
                (model, Cmd.none)



-- Subscriptions

subscriptions : Model -> Sub Msg
subscriptions model = 
    WebSocket.listen url NewMessage


-- View

view : Model -> Html Msg
view model =
    button [onClick Send] [text "send thing"]



main =
    Html.program
        { init = init
        , view = view
        , update = update
        , subscriptions = subscriptions
        }
