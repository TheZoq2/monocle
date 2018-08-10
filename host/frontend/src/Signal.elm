module Signal exposing 
    ( continuousRead
    , isFallingEdge
    , isRisingEdge
    , edgeTrigger
    )


continuousRead : Bool -> Bool -> Bool
continuousRead _ _ =
    True

isFallingEdge : Bool -> Bool -> Bool
isFallingEdge old new =
    old == True && new == False

isRisingEdge : Bool -> Bool -> Bool
isRisingEdge old new =
    old == False && new == True


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


{-|
  Finds the length of the gap at `gapLocation` if gapLocation
  is larger than the max time in the list `Nothing` is returned
-}
gapLength : Float -> List (Float, Bool) -> Maybe Float
gapLength gapLocation values =
    let
        -- Finds the last value before location along with when
        -- that value was set
        recurseFn : (Bool, Float) -> List (Float, Bool) -> Maybe Float
        recurseFn (lastValue, lastChange) values =
            case values of
                (time, value) :: rest ->
                    -- If the last value is the same as the current value,
                    -- do nothing
                    if lastValue == value then
                        recurseFn (value, time) rest
                    -- Otherwise, check if the time is up, if so return
                    -- the interval length, otherwise recurse
                    else
                        if time > gapLocation then
                            Just (time - lastChange)
                        else
                            recurseFn (lastValue, lastChange) rest
                _ ->
                    Nothing
    in
        case values of
            (time, value) :: rest ->
                recurseFn (value, 0) rest
            _ ->
                Nothing

