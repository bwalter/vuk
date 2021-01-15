import QtQuick 2.9

Item {
    function increaseFontSize() {
        _fontFactor = _fontFactor * 1.2;
    }
    readonly property bool canIncreaseFontSize: _fontFactor < 2.2

    function decreaseFontSize() {
        _fontFactor = _fontFactor * 0.8;
    }
    readonly property bool canDecreaseFontSize: _fontFactor >= 0.65
    
    property real _fontFactor: 1.0

    readonly property string mainFontFamily: mainFont.name

    readonly property real itemTitleFontSize: 10 * _fontFactor
    readonly property real itemMemberFontSize: 9 * _fontFactor
    readonly property real listViewItemFontSize: 10

    readonly property string toolFontFamily: toolFont.name
    readonly property string toolTextColor: "#3a3a3a"
    readonly property real toolIconFontSize: 14

    readonly property color listViewItemBackgroundColor: "white"
    readonly property color listViewItemHighlightColor: "blue"
    readonly property color listViewItemTextColor: "black"
    readonly property color listViewItemAlternativeTextColor: "#444444"
    readonly property color listViewItemHighlightedTextColor: "white"
    readonly property color listViewItemHighlightedAlternativeTextColor: "#bbbbbb"
    
    readonly property color diagramItemBorderColor: "white"
    readonly property real diagramItemBorderWidth: 4
    readonly property color diagramItemTitleColor: "white"
    readonly property color diagramItemMemberColor: "white"
    readonly property color diagramItemHighlightColor: "#ec3c13"
    
    FontLoader {
        id: mainFont
        source: "qrc:/vuk/src/ui_quick/fonts/segoeui.ttf"
    }

    FontLoader {
        id: toolFont
        source: "qrc:/vuk/src/ui_quick/fonts/SegMDL2.ttf"
    }

    function colorOfItemType(itemType) {
        switch (itemType) {
            //case "Interface": return "#FFCAB0";
            //case "Interface": return "#FF6D3A";
            //case "Interface": return "#ff9726";
            //case "Interface": return "#ed4c14";
            case "Interface": return "#ed7b14";
            //case "Enum": return "#B0D9FF";
            //case "Enum": return "#17C5CC";
            case "Enum": return "#71b32b";
            //case "Struct": return "#F9EC82";
            //case "Struct": return "#FEDB67";
            case "Struct": return "#1b91db";
            default: return "#EEEEEE";
        }
    }
}