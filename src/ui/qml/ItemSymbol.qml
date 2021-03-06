import QtQuick 2.9
import QtQuick.Controls 2.15
import "./style"

Rectangle {
    width: 20
    height: width
    radius: width / 2
    border.color: "black"
    color: Style.colorOfItemType(itemType)
    
    property string itemType
    
    Label {
        id: label
        anchors.centerIn: parent
        text: switch (itemType) {
            case "Interface": "i"; break;
            case "Enum": "E"; break;
            case "Struct": "P"; break;
            default: "";
        }
        color: parent.border.color
    }
}
