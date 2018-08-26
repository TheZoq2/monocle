module View exposing (view)

-- Standard library imports

import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Svg
import Svg.Attributes

-- External imports

import List.Extra
import Mouse
import Graph
import Style

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
            button
                [ onClick (msg alternative)
                , (if alternative == current then
                        Style.class [Style.SelectedButton]
                    else 
                        Style.class []
                    )
                ]
                [ text <| nameFunction alternative
                ]
        )
        choises


drawGraph : (Int, Int) -> (Float, Float) -> List (Float, Bool) -> Html Msg
drawGraph (viewWidth, viewHeight) valueRange readingList =
    div [Style.class [Style.Graph], Mouse.onDown GraphClicked]
        [ Svg.svg
            [ Svg.Attributes.viewBox <| "0 0 " ++ (toString viewWidth) ++ " " ++ (toString viewHeight)
            , Svg.Attributes.width <| toString viewWidth ++ "px"
            , Svg.Attributes.height <| toString viewHeight ++ "px"
            ]
            [ Graph.drawHorizontalLines (viewWidth, viewHeight) (0,1) 1
            , Graph.drawGraph (viewWidth, viewHeight) (0,1) valueRange
              <| List.map (\(time, val) -> if val then (time, 1) else (time, 0)) readingList
            ]
        ]


view : Model -> Html Msg
view model =
    let
        readings = List.map stepPreprocessor 
            <| readingsToChannels (model.readings ++ [model.currentReading])


        valueRange = edgeTrigger
            (trigFunction model.triggerMode)
            (toMicroseconds model.timeSpan)
            (Maybe.withDefault [] <| List.Extra.getAt model.triggerChannel readings)

        graphViewX = 600
        graphViewY = 50
        graphViewSize = (graphViewX, graphViewY)

        -- Calculate the graph offset
        graphOffset = 
            model.graphOffset / graphViewX * (Tuple.second valueRange - Tuple.first valueRange)

        (displayMin, displayMax) = valueRange
        displayRange = (displayMin - graphOffset, displayMax - graphOffset)

        graphFunction = drawGraph graphViewSize displayRange


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
        contentContainer model
            <|  [ div [Style.class [Style.ButtonRow]]
                    [ triggerModeRow
                    , triggerChannelSelector
                    , timeSpanSelection
                    ]
                ]
                ++
                (List.map graphFunction readings)
                ++
                buttonRow



contentContainer : Model -> List (Html Msg) -> Html Msg
contentContainer model children =
    let
        eventListeners =
            if model.mouseDragReceiver == Nothing then
                []
            else
                [ Mouse.onMove MouseGlobalMove
                , Mouse.onUp MouseGlobalUp
                , Mouse.onLeave MouseGlobalLeave
                ]
    in
    div
        ( [ Style.class [Style.Content]
          ]
          ++
          eventListeners
        )
        children

