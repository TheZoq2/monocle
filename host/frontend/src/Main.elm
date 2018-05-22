module Main exposing (..)

-- Standard library imports
import Html
import WebSocket

-- Library imports
import Json.Decode

-- Main imports
import View exposing (view)
import Model exposing (Model, init)
import Msg exposing (Msg(..))

import Types exposing
    ( Message(..),
    Reading,
    messageDecoder,
    readingsToChannels,
    TriggerMode(..),
    triggerModeSymbol,
    allTriggerModes
    )


url : String
url = "ws://localhost:8765"

-- Update function

update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
    case msg of
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



main =
    Html.program
        { init = init
        , view = view
        , update = update
        , subscriptions = subscriptions
        }
