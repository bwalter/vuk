import QtQuick 2.9
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

Rectangle {
    id: diagramItem
    property var node
    property var connectors: []  // TODO: separate left/right?
    property int position: 1
    property int highlightedMember: -1
    property var memberFilter: []
    property var expanded: false
    
    enum Visibility {
        ShowAll,
        ShowWithFilter,
        ShowNone
    }
    
    property color backgroundColor: {
        if (position === 1) return colors.colorOfItemType(node.item.item_type);
        return "white";
    }

    color: "white"
    border.color: "black"
    border.width: 1
    height: column.height
    
    gradient: Gradient {
        GradientStop { position: 0.0; color: diagramItem.backgroundColor }
        GradientStop { position: 1.0; color: "#ffffff" }
    }
    
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

            RowLayout {
                anchors.fill: parent
                
                Item {
                    Layout.fillWidth: true
                }

                ItemSymbol {
                    color: "white"
                    itemType: diagramItem.node.item.item_type
                }

                Label {
                    id: titleLabel
                    Layout.alignment: Qt.AlignVCenter

                    color: "black"
                    text: diagramItem.node.item.name
                    font.pointSize: 10
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
                    if (!expanded) expanded = true;
                    else openClicked();
                }
            }
        }
        
        Rectangle {
            anchors { left: parent.left; right: parent.right }
            height: 1
            color: "black"
        }
        
        Column {
            id: memberColumn
            anchors { left: parent.left; right: parent.right }
            anchors { leftMargin: 20; rightMargin: 20 }
            height: implicitHeight
            clip: true
            
            Item { visible: expanded || memberFilter.length > 0; width: 10; height: 10 }
            
            Repeater {
                model: diagramItem.node.item.members

                Label {
                    anchors { left: parent.left; right: parent.right }
                    height: expanded || passFilter ? implicitHeight : 0
                    opacity: expanded && memberFilter.length > 0 && !passFilter ? 0.2 :
                        expanded || passFilter ? 1.0 : 0
                    text: modelData.text
                    color: memberMouseArea.containsMouse ? "blue" : "black"
                    maximumLineCount: 1
                    wrapMode: Text.Wrap
                    font.pointSize: 9
                    elide: Text.ElideRight

                    property bool passFilter: memberFilter.includes(modelData.index)
                    
                    Behavior on height {
                        enabled: diagram.ready
                        NumberAnimation { duration: diagram.ready ? 250 : 0 }
                    }

                    Behavior on opacity {
                        enabled: diagram.ready
                        NumberAnimation { duration: diagram.ready ? 250 : 0 }
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

            Item { visible: expanded; width: 10; height: 10 }
        }

        Label {
            id: expandCollapse
            anchors { horizontalCenter: parent.horizontalCenter }
            visible: memberFilter.length > 0
            height: visible ? implicitHeight : 0
            text: expanded ? "\u25B2" : "..."
            color: "black"
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