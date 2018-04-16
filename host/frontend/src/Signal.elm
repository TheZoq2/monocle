module Signal exposing (triggerFallingEdge)


isFallingEdge : Bool -> Bool -> Bool
isFallingEdge old new =
    old == True && new == False

isRisingEdge : Bool -> Bool -> Bool
isRisingEdge old new =
    old == False && new == True

-- triggerFallingEdge : Float -> List (Float Bool) -> List (Float Bool)
-- triggerFallingEdge displayRange data =
-- 


lastTrig : (Bool -> Bool -> Bool) -> List (Float, Bool) -> Maybe Float
lastTrig trigfn reversedReadings =
    let
        inner : (Float, Bool) -> List (Float, Bool) -> Maybe Float
        inner (prevTime, prevVal) readings =
            case readings of
                ((firstTime, firstVal), rest) ->
                    if trigfn firstVal prevVal then
                        Just prevTime
                    else
                        inner (firstTime, firstVal) rest
                _ -> Nothing
    in
        Maybe.map2 (\first rest -> inner first rest)

edgeTrigger : (Bool -> Bool) -> Float -> List (Float Bool) -> (Float, Float)
edgeTrigger trigFn valueRange readings =
    let
        reversedReadings = List.reverse readings
    in
        

