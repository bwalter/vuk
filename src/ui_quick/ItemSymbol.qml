import QtQuick 2.9
import QtQuick.Controls 2.15

Rectangle {
    width: label.font.pointSize * 2.8
    height: width
    radius: width / 2
    border.color: "black"
    color: style.colorOfItemType(itemType)
    
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
