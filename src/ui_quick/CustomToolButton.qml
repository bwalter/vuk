import QtQuick 2.9
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

ToolButton {
    id: control

    property string iconCode: ""
    
    readonly property Item style: window.style
    
    //Binding {
    //    target: background
    //    property: "implicitHeight"
    //    value: 100
    //}

    contentItem: RowLayout {
        id: rowLayout
        anchors.verticalCenter: parent.verticalCenter
        opacity: enabled ? 1.0 : 0.3

        readonly property color textColor: control.style.toolTextColor

        Text {
            Layout.alignment: Qt.AlignVCenter
            font.family: control.style.toolFontFamily 
            font.pointSize: control.style.toolIconFontSize
            text: control.iconCode
            color: parent.textColor
        }

        Text {
            Layout.alignment: Qt.AlignVCenter
            font.family: control.style.mainFontFamily 
            text: control.text
            elide: Text.ElideRight
            color: parent.textColor
        }
    }
    
    background.implicitHeight: 80
}