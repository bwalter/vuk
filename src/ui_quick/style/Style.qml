pragma Singleton
import QtQuick 2.9

QtObject {
    function increaseFontSize() {
        fontFactor = fontFactor * 1.2;
    }
    readonly property bool canIncreaseFontSize: fontFactor < 2.2

    function decreaseFontSize() {
        fontFactor = fontFactor * 0.8;
    }
    readonly property bool canDecreaseFontSize: fontFactor >= 0.65

    property int transitionDuration: 250
    
    property real fontFactor: 1.0

    readonly property string mainFontFamily: mainFont.name

    readonly property real itemTitleFontSize: 10 * fontFactor
    readonly property real itemMemberFontSize: 9 * fontFactor
    readonly property real listViewItemFontSize: 10

    readonly property string toolFontFamily: toolFont.name
    readonly property string toolTextColor: "#3a3a3a"
    readonly property real toolIconFontSize: 12

    readonly property color listViewItemBackgroundColor: "white"
    readonly property color listViewItemHighlightColor: "blue"
    readonly property color listViewItemTextColor: "black"
    readonly property color listViewItemAlternativeTextColor: "#444444"
    readonly property color listViewItemHighlightedTextColor: "white"
    readonly property color listViewItemHighlightedAlternativeTextColor: "#bbbbbb"
    
    readonly property real diagramItemRadius: 10
    readonly property real diagramItemBorderWidth: 2
    readonly property real diagramItemTopPadding: 6
    readonly property real diagramItemBottomPadding: 10
    readonly property color diagramItemBackgroundColor: "#ffffff"
    readonly property color diagramItemMemberColor: "#6c6c6c"
    readonly property color diagramItemHighlightColor: "#ec3c13"

    readonly property real connectorWidth: 2
    readonly property real connectorArrowLength: 12
    readonly property color connectorColor: "#6c6c6c"
    
    readonly property FontLoader mainFont: FontLoader {
        source: "qrc:/vuk/src/ui_quick/fonts/segoeui.ttf"
    }

    readonly property FontLoader toolFont: FontLoader {
        source: "qrc:/vuk/src/ui_quick/fonts/SegMDL2.ttf"
    }

    function colorOfItemType(itemType) {
        switch (itemType) {
            //case "Interface": return "#ed7b14";
            case "Interface": return "#ec3c13";
            case "Enum": return "#71b32b";
            case "Struct": return "#1b91db";
            default: return "#EEEEEE";
        }
    }
}
