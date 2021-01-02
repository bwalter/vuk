import QtQuick 2.9
import QtQuick.Controls 2.15

Item {
    property Item itemFrom
    property Item itemTo
    property var indicesFrom: []

    property Item leftItem: itemTo.x > itemFrom.x ? itemFrom : itemTo
    property Item rightItem: leftItem === itemFrom ? itemTo : itemFrom

    property Item topItem: (itemTo.y + itemTo.height / 2) > (itemFrom.y + itemFrom.height / 2) ? itemFrom : itemTo
    property Item bottomItem: topItem === itemFrom ? itemTo : itemFrom

    x: leftItem.x + leftItem.width
    y: topItem.y + topItem.height / 2
    width: rightItem.x - (leftItem.x + leftItem.width)
    height: Math.max(1, (bottomItem.y + bottomItem.height / 2 - 1) - (topItem.y + topItem.height / 2))
    
    // ---
    //    |
    //     ---

    // ---
    Rectangle {
        x: 0
        y: parent.leftItem === parent.bottomItem ? parent.height - 1 : 0
        height: 1
        width: parent.width / 2 - 1
        color: "black"
    }

    //    |
    Rectangle {
        x: parent.width / 2 - 1
        y: 0
        height: parent.height
        width: 1
        color: "black"
    }

    //     ---
    Rectangle {
        x: parent.width / 2 - 1
        y: parent.rightItem === parent.bottomItem ? parent.height - 1 : 0
        height: 1
        width: parent.width / 2
        color: "black"
    }
    
    // Arrow
    Rectangle {
        x: itemTo === parent.rightItem ? parent.width : 0
        y: itemTo === parent.topItem ? 0 : parent.height - 1
        width: 22
        height: 1
        color: "black"
        rotation: itemTo === parent.rightItem ? -145 : 35
        transformOrigin: Item.Left
    }

    Rectangle {
        x: itemTo === parent.rightItem ? parent.width : 0
        y: itemTo === parent.topItem ? 0 : parent.height - 1
        width: 22
        height: 1
        color: "black"
        rotation: itemTo === parent.rightItem ? -215 : -35
        transformOrigin: Item.Left
    }
}