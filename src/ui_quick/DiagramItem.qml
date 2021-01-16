import QtQuick 2.9
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import "./style"

Rectangle {
    id: diagramItem
    property var node
    property var connectors: []  // TODO: separate left/right?
    property int position: 1
    property int highlightedMember: -1
    property var memberFilter: []
    property var expanded: false
    
    readonly property color foregroundColor: {
        //if (position === 1) return Style.diagramItemBorderColor;
        return Style.colorOfItemType(node.item.item_type);
    }

    readonly property color backgroundColor: {
        //if (position === 1) return Style.colorOfItemType(node.item.item_type);
        return Style.diagramItemBackgroundColor;
    }
    
    enum Visibility {
        ShowAll,
        ShowWithFilter,
        ShowNone
    }

    radius: Style.diagramItemRadius
    
    border.color: foregroundColor
    border.width: Style.diagramItemBorderWidth
    height: column.height
    
    color: backgroundColor
    
    signal openClicked()
    
    Component.onDestruction: clearConnectors()
    
    function createConnector(itemFrom, itemTo) {
        const connector = connectorComponent.createObject(diagram, { itemFrom, itemTo });
        connectors.push(connector);
    }
    
    function clearConnectors() {
        connectors.forEach(c => c.destroy());
        connectors = [];
    }
    
    Column {
        id: column
        anchors { left: parent.left; right: parent.right }

        Item {
            id: titleItem
            anchors { left: parent.left; right: parent.right }
            height: titleLabel.implicitHeight * 1.5
            clip: true

            Rectangle {
                id: titleBackgroundRect
                anchors.fill: parent
                visible: position === 1
                color: position === 1 ? diagramItem.foregroundColor : diagramItem.backgroundColor
                radius: diagramItem.radius
                border.color: diagramItem.foregroundColor
                border.width: Style.diagramItemBorderWidth
            }

            Rectangle {
                anchors { top: parent.verticalCenter; bottom: parent.bottom; left: parent.left; right: parent.right }
                visible: position === 1
                color: titleBackgroundRect.color
            }

            RowLayout {
                anchors.fill: parent
                
                Item {
                    Layout.fillWidth: true
                }

                ItemSymbol {
                    color: titleLabel.color
                    border.color: titleBackgroundRect.color
                    itemType: diagramItem.node.item.item_type
                }

                Label {
                    id: titleLabel
                    Layout.alignment: Qt.AlignVCenter

                    color: position === 1 ? diagramItem.backgroundColor : diagramItem.foregroundColor
                    text: diagramItem.node.item.name
                    font.family: Style.mainFontFamily
                    font.pointSize: Style.itemTitleFontSize
                    font.bold: true
                    verticalAlignment: Qt.AlignVCenter
                    elide: Text.ElideRight
                }

                Item {
                    Layout.fillWidth: true
                }
            }
            
            MouseArea {
                id: mouseArea
                anchors.fill: parent
                acceptedButtons: Qt.LeftButton | Qt.RightButton
                onClicked: {
                    if (mouse.buttons & Qt.RightButton) openClicked();
                    else if (mouse.button !== Qt.LeftButton) return;

                    if (mouse.modifiers & Qt.ControlModifier) openClicked();
                    else if (mouse.modifiers & Qt.MetaModifier) openClicked();
                    else expanded = !expanded;
                }
            }
        }
        
        Rectangle {
            anchors { left: parent.left; right: parent.right }
            opacity: expanded || memberFilter.length > 0 ? 1 : 0
            height: Style.diagramItemBorderWidth
            color: diagramItem.foregroundColor

            // Must match the height animation of the member labels
            Behavior on opacity { NumberAnimation { duration: Style.transitionDuration } }
        }
        
        Column {
            id: memberColumn
            anchors { left: parent.left; right: parent.right }
            anchors { leftMargin: 20; rightMargin: 20 }
            height: implicitHeight
            clip: true
            
            Item {
                width: 10
                height: expanded || memberFilter.length > 0 ? Style.diagramItemTopPadding : 0

                // Must match the height animation of the member labels
                Behavior on height { NumberAnimation { duration: Style.transitionDuration } }
            }
            
            Repeater {
                id: repeater
                model: diagramItem.node.item.members

                Label {
                    anchors { left: parent.left; right: parent.right }
                    height: expanded || passFilter ? implicitHeight : 0
                    opacity: expanded && memberFilter.length > 0 && !passFilter ? 0.2 :
                        expanded || passFilter ? 1.0 : 0
                    text: modelData.text
                    //color: memberMouseArea.containsMouse ? Style.diagramItemHighlightColor : diagramItem.foregroundColor
                    color: Style.diagramItemMemberColor
                    maximumLineCount: 1
                    wrapMode: Text.Wrap
                    font.family: Style.mainFontFamily
                    font.pointSize: Style.itemTitleFontSize
                    elide: Text.ElideRight

                    property bool passFilter: memberFilter.includes(modelData.index)
                    
                    Behavior on height {
                        enabled: diagram.ready
                        NumberAnimation { duration: diagram.ready ? Style.transitionDuration : 0 }
                    }

                    Behavior on opacity {
                        enabled: diagram.ready
                        NumberAnimation { duration: diagram.ready ? Style.transitionDuration : 0 }
                    }

                    MouseArea {
                        id: memberMouseArea
                        anchors.fill: parent
                        hoverEnabled: true
                        onContainsMouseChanged: {
                            if (containsMouse) diagramItem.highlightedMember = modelData.index
                            else if (diagramItem.highlightedMember === modelData.index) diagramItem.highlightedMember = -1
                        }
                    }
                }
            }

            Item {
                width: 10
                height: expanded ? Style.diagramItemBottomPadding : 0

                // Must match the height animation of the member labels
                Behavior on height { NumberAnimation { duration: Style.transitionDuration } }
            }
        }

        Label {
            id: expandCollapse
            anchors { horizontalCenter: parent.horizontalCenter }
            visible: memberFilter.length > 0
            height: visible ? implicitHeight : 0
            text: expanded ? "\u25B2" : "..."
            color: diagramItem.foregroundColor
            font.pointSize: 9
            
            MouseArea {
                anchors.fill: parent
                onClicked: expanded = !expanded
            }
        }

        Item { visible: expandCollapse.visible; width: 10; height: 10 }
    }
    
    Component {
        id: connectorComponent

        Connector {
            opacity: diagramItem.opacity
        }
    }
}
