port module Style exposing (..)

import Css exposing (..)
import Css.File exposing (..)
import Css.Elements exposing (..)
import Html.Attributes exposing (style)
import Html.CssHelpers
import Html

port files : CssFileStructure -> Cmd msg



toStyle : List Css.Mixin -> Html.Attribute msg
toStyle =
    Css.asPairs >> Html.Attributes.style



--Css helpers


{ id, class, classList } =
    Html.CssHelpers.withNamespace ""


type CssClasses
    = Content
    | Graph
    | ButtonRow
    | SelectedButton



--Some very common parameters



--The style to apply to all files in the project


globalStyle : List Css.Stylesheet
globalStyle =
    [(stylesheet)
        [ body
            [ fontFamilies ["sans-serif"]
            ]
        , Css.class Content
            [ width <| px 960
            , margin2 (px 0) auto
            ]
        , Css.class Graph
            [ margin2 (px 10) (px 0)
            , Css.descendants
                [ polyline
                    [ property "stroke-width" "2px" ]
                ]
            ]
        , Css.class ButtonRow
            [ margin2 (px 5) zero
            , displayFlex
            , Css.children
                [ div
                    [ padding2 (px 7) (px 0)
                    , float left
                    , margin2 (px 0) (px 10)
                    , Css.children
                        [ label 
                            [ display block
                            , fontSize (px 13)
                            , margin2 (px 3) (px 0)
                            ]
                        ]
                    ]
                ]
            ]
        , button
            [ border (px 0)
            , borderBottom3 (px 4) solid (rgba 0 0 0 0)
            , borderRadius (px 3)
            , padding2 (px 4) (px 6)
            , paddingBottom (px 0)
            , boxShadow4 (px 2) (px 2) (px 2) (hex "aaa")
            , margin (px 3)
            , hover
                [ borderBottomColor (rgb 207 161 227)]
            ]
        , Css.class SelectedButton
            [ borderBottomColor (hex "863aa4")]
        ]
    ]




cssFiles : CssFileStructure
cssFiles =
    toFileStructure [ ( "output/css/GlobalStyle.css", Css.File.compile globalStyle ) ]

main : CssCompilerProgram
main =
    Css.File.compiler files cssFiles
