import QtQuick 2.9
import QtQuick.Controls 2.15

Rectangle {
    width: 30
    height: width
    radius: width / 2
    border.color: "black"
    color: colors.colorOfItemType(itemType)
    
    property string itemType
    
    Label {
        anchors.centerIn: parent
        text: switch (itemType) {
            case "Interface": "i"; break;
            case "Enum": "E"; break;
            case "Struct": "P"; break;
            default: "";
        }
        color: "black"
    }
}
