import QtQuick 2.9
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import "./style"

ToolButton {
    id: control
    text: action.text
    
    contentItem: RowLayout {
        id: rowLayout
        anchors.verticalCenter: parent.verticalCenter
        opacity: enabled ? 1.0 : 0.3

        readonly property color textColor: Style.toolTextColor

        Text {
            Layout.alignment: Qt.AlignVCenter
            font.family: Style.toolFontFamily 
            font.pointSize: Style.toolIconFontSize
            text: control.action.iconCode
            color: parent.textColor
        }

        Text {
            Layout.alignment: Qt.AlignVCenter
            font.family: Style.mainFontFamily 
            text: control.text
            elide: Text.ElideRight
            color: parent.textColor
        }
    }
    
    background.implicitHeight: 40
}
