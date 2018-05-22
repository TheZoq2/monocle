module View exposing (view)

-- Standard library imports

import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Svg
import Svg.Attributes

-- External imports

import List.Extra
import Graph

-- Main imports

import Msg exposing (Msg(..))
import Model exposing (Model)

-- Project imports

import Types exposing 
    ( TriggerMode(..)
    , readingsToChannels
    , allTriggerModes
    , triggerModeSymbol
    )
import TimeUnits exposing 
    ( toMicroseconds
    , allTimeUnits
    , timeUnitString
    , TimeUnit
    )
import Signal exposing (continuousRead, isRisingEdge, isFallingEdge, edgeTrigger)





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
