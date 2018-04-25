module Signal exposing (continuousRead, isFallingEdge, isRisingEdge, edgeTrigger)


continuousRead : Bool -> Bool -> Bool
continuousRead _ _ =
    True

isFallingEdge : Bool -> Bool -> Bool
isFallingEdge old new =
    old == True && new == False

isRisingEdge : Bool -> Bool -> Bool
isRisingEdge old new =
    old == False && new == True

-- triggerFallingEdge : Float -> List (Float Bool) -> List (Float Bool)
-- triggerFallingEdge displayRange data =
-- 


getLastTrig : (Bool -> Bool -> Bool) -> List (Float, Bool) -> Maybe Float
getLastTrig trigfn reversedReadings =
    let
        inner : (Float, Bool) -> List (Float, Bool) -> Maybe Float
        inner (prevTime, prevVal) readings =
            case readings of
                (firstTime, firstVal) :: rest ->
                    if trigfn firstVal prevVal then
                        Just prevTime
                    else
                        inner (firstTime, firstVal) rest
                _ -> Nothing
    in
        Maybe.withDefault Nothing
            <| Maybe.map2
                (\first rest -> inner first rest)
                (List.head reversedReadings)
                (List.tail reversedReadings)

edgeTrigger : (Bool -> Bool -> Bool) -> Float -> List (Float, Bool) -> (Float, Float)
edgeTrigger trigFn valueRange readings =
    let
        reversedReadings = List.reverse readings

        maxTime = Maybe.withDefault valueRange
            <| List.maximum
            <| List.map Tuple.first readings

        lastTrig = Maybe.withDefault maxTime
            <| getLastTrig trigFn reversedReadings

        end = min (lastTrig + valueRange/2) maxTime
    in
        (end - valueRange, end)
