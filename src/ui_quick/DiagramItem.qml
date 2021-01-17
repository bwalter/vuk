import QtQuick 2.9
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import "./style"

Rectangle {
    id: diagramItem
    property var node
    property var connectors: []  // TODO: separate left/right?
    property int position: 1
    property int highlightedMemberIndex: -1
    readonly property var highlightedMember: {
        if (highlightedMemberIndex < 0) return null;
        return node.item.members[highlightedMemberIndex];
    }
    readonly property alias containsMouse: mouseArea.containsMouse

    property var memberFilter: null

    property var explicitExpanded: undefined  // manually changed by user
    property bool implicitExpanded: true  // set by diagram
    readonly property bool expanded: explicitExpanded === undefined ? implicitExpanded : explicitExpanded;

    property var explicitOnlyShowRelevantMembers: undefined  // manually changed by user
    property bool implicitOnlyShowRelevantMembers: false  // set by diagram
    readonly property bool onlyShowRelevantMembers: explicitOnlyShowRelevantMembers === undefined ? implicitOnlyShowRelevantMembers : explicitOnlyShowRelevantMembers;
    
    readonly property int memberCount: node.item.members ? node.item.members.length : 0
    readonly property real heightAfterAnimation: {
        if (!expanded) return titleItem.height;
        
        const memberCountDisplayed = memberFilter && onlyShowRelevantMembers ? memberFilter.length : memberCount;
        return titleItem.height + separator.height
            + (memberCountDisplayed * memberTextMetrics.boundingRect.height)  // members
            + (Style.diagramItemTopPadding + Style.diagramItemBottomPadding)  // padding
            + (memberFilter ? 10 : 0);  // expand/collapse
    }
    
    readonly property color foregroundColor: {
        //if (position === 1) return Style.diagramItemBorderColor;
        return Style.colorOfItemType(node.item.item_type);
    }

    readonly property color backgroundColor: {
        //if (position === 1) return Style.colorOfItemType(node.item.item_type);
        return Style.diagramItemBackgroundColor;
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
        return connector;
    }
    
    function clearConnectors() {
        connectors.forEach(c => c.destroy());
        connectors = [];
    }
    
    TextMetrics {
        id: memberTextMetrics
        text: "XXX"
        font.family: Style.mainFontFamily
        font.pointSize: Style.itemTitleFontSize
    }
    
    Column {
        id: column
        anchors { left: parent.left; right: parent.right }
        spacing: 0

        Item {
            id: titleItem
            anchors { left: parent.left; right: parent.right }
            height: titleLabel.implicitHeight * 1.5
            clip: true

            Rectangle {
                id: titleBackgroundRect
                anchors { top: parent.top; left: parent.left; right: parent.right }
                height: column.height
                visible: position === 1
                color: position === 1 ? diagramItem.foregroundColor : diagramItem.backgroundColor
                radius: diagramItem.radius
                border.color: diagramItem.foregroundColor
                border.width: Style.diagramItemBorderWidth
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

                    color: position === 1 ? diagramItem.backgroundColor :
                        diagramItem.containsMouse ? Qt.darker(diagramItem.foregroundColor) :
                        diagramItem.foregroundColor
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
                hoverEnabled: true
                acceptedButtons: Qt.LeftButton | Qt.RightButton
                onClicked: {
                    if (mouse.buttons & Qt.RightButton) openClicked();
                    else if (mouse.button !== Qt.LeftButton) return;

                    if (mouse.modifiers & Qt.ControlModifier) openClicked();
                    else if (mouse.modifiers & Qt.MetaModifier) openClicked();
                    else explicitExpanded = !expanded;
                }
            }
        }
        
        Rectangle {
            id: separator
            anchors { left: parent.left; right: parent.right }
            opacity: expanded ? 1 : 0
            height: Style.diagramItemBorderWidth
            color: diagramItem.foregroundColor

            // Must match the height animation of the member labels
            Behavior on opacity {
                enabled: diagram.implicitAnimationsEnabled
                NumberAnimation { duration: Style.transitionDuration }
            }
        }
        
        Column {
            id: memberColumn
            anchors { left: parent.left; right: parent.right }
            anchors { leftMargin: 20; rightMargin: 20 }
            spacing: 0
            clip: true
            
            Item {
                width: 10
                height: expanded ? Style.diagramItemTopPadding : 0

                // Must match the height animation of the member labels
                Behavior on height {
                    enabled: diagram.implicitAnimationsEnabled
                    NumberAnimation { duration: Style.transitionDuration }
                }
            }
            
            Repeater {
                id: repeater
                model: diagramItem.node.item.members
                
                Label {
                    readonly property bool shouldBeShown: !!expanded && (!onlyShowRelevantMembers || passFilter)
                    readonly property bool passFilter: memberFilter === null || memberFilter.includes(modelData.index)

                    anchors { left: parent.left; right: parent.right }
                    height: shouldBeShown ? memberTextMetrics.boundingRect.height : 0
                    opacity: !expanded ? 0 :
                        !memberFilter ? 1 :
                        passFilter ? 1 : 0.35
                    text: modelData.text
                    color: memberMouseArea.containsMouse ? Qt.darker(diagramItem.foregroundColor) /*Style.diagramItemHighlightColor*/ : diagramItem.foregroundColor
                    //color: Style.diagramItemMemberColor
                    maximumLineCount: 1
                    wrapMode: Text.Wrap
                    font: memberTextMetrics.font
                    elide: Text.ElideRight
                    
                    Behavior on height {
                        enabled: diagram.implicitAnimationsEnabled
                        NumberAnimation { duration: diagram.ready ? Style.transitionDuration : 0 }
                    }

                    Behavior on opacity {
                        enabled: diagram.implicitAnimationsEnabled
                        NumberAnimation { duration: diagram.ready ? Style.transitionDuration : 0 }
                    }

                    MouseArea {
                        id: memberMouseArea
                        enabled: position === 1
                        anchors.fill: parent
                        hoverEnabled: true
                        onContainsMouseChanged: {
                            if (containsMouse) diagramItem.highlightedMemberIndex = modelData.index
                            else if (diagramItem.highlightedMemberIndex === modelData.index) diagramItem.highlightedMemberIndex = -1
                        }
                    }
                }
            }

            Item {
                width: 10
                height: expanded ? Style.diagramItemBottomPadding : 0

                // Must match the height animation of the member labels
                Behavior on height {
                    enabled: diagram.implicitAnimationsEnabled
                    NumberAnimation { duration: Style.transitionDuration }
                }
            }
        }

        Item {
            id: expandCollapse
            anchors { left: parent.left; right: parent.right }
            height: expanded && memberFilter && memberFilter.length !== memberCount ? 20 : 0
            opacity: expanded && memberFilter && memberFilter.length !== memberCount ? 1 : 0

            // Must match the height animation of the member labels
            Behavior on height {
                enabled: diagram.implicitAnimationsEnabled
                NumberAnimation { duration: Style.transitionDuration }
            }
            Behavior on opacity {
                enabled: diagram.implicitAnimationsEnabled
                NumberAnimation { duration: Style.transitionDuration }
            }

            Label {
                anchors { horizontalCenter: parent.horizontalCenter; bottom: parent.verticalCenter }
                text: !!onlyShowRelevantMembers ? "..." : "\u25B2"
                color: diagramItem.foregroundColor
                font.pointSize: 9
            }

            MouseArea {
                anchors.fill: parent
                onClicked: explicitOnlyShowRelevantMembers = !onlyShowRelevantMembers
            }
        }
    }
    
    Component {
        id: connectorComponent

        Connector {
            //opacity: diagramItem.opacity
        }
    }
}
