import QtQuick 2.9
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import "./style"

Rectangle {
    id: diagram
    color: "white"
    
    // Initial size = scrollview size
    width: diagramScrollView.width
    height: diagramScrollView.height
    
    property real horizontalPadding: Math.max(25, Math.min(50, diagramScrollView.width / 20))
    property real horizontalSpacing: Math.max(50, Math.min(100, diagramScrollView.width / 10))
    property real verticalSpacing: 15
    property real verticalPadding: 25
    property real mainItemWidth: Math.max(150, Math.min(400, diagramScrollView.width / 3.5))
    property real otherItemWidth: Math.max(100, Math.min(300, (diagramScrollView.width - mainItemWidth - horizontalPadding * 2 - horizontalSpacing * 2) / 2))
    
    property var leftItems: null
    property var rightItems: null

    property var highlightedItem: null
    readonly property var highlightedMember: {
        return mainItem && mainItem.highlightedMember;
    }

    readonly property bool implicitAnimationsEnabled: ready

    property bool _layouting: false
    property bool _adjustingItems: false
    property bool _updatingExplicitExpanded: false
    property bool _updatingExplicitShowRelevantMembers: false
        
    onMainItemChanged: {
        scale = 0;
        delayedInitTimer.start();
        appearInAnimation.start();
    }
    
    property Item mainItem: null
    property bool ready: false

    ScaleAnimator {
        id: appearInAnimation
        target: diagram
        from: 0.0
        to: 1.0
        duration: Style.transitionDuration
    }
    
    Timer {
        id: delayedInitTimer
        interval: 500
        repeat: false
        onTriggered: diagram.ready = true
    }
    
    Connections {
        target: diagramScrollView

        function onWidthChanged() {
            adjustItemStateForAvailableSpace();
            diagram.layout();
        }

        function onHeightChanged() {
            adjustItemStateForAvailableSpace();
            diagram.layout();
        }
    }
    
    function setMainNode(mainNode) {
        if (mainItem && mainNode && mainNode.item.key === mainItem.node.item.key) return;

        ready = false;
        
        // Destroy current main item and sub items
        destroySubItems(diagram.leftItems);
        destroySubItems(diagram.rightItems);
        if (mainItem) mainItem.destroy();
        
        if (!mainNode) return;

        // Create new one
        mainItem = createItem(mainNode, { position: 1, explicitExpanded: true });
        
        // Expand
        expandLeft();
        expandRight();
        
        // Layout diagram
        adjustItemStateForAvailableSpace();
        layout();

        vuk.currentItem = mainNode.item;
    }
                    
    function expandLeft() {
        const edges = JSON.parse(vuk.get_dependent_edges(mainItem.node.item.key));
        diagram.destroySubItems(diagram.leftItems);
        
        diagram.leftItems = edges.map(edge => {
            const subItem = diagram.createItem(edge.to, { position: 0 });
            subItem.createConnector(subItem, mainItem);
            subItem.memberFilter = edge.from_indices;
            return subItem;
        });
    }

    function expandRight() {
        if (!mainItem) return;

        const edges = JSON.parse(vuk.get_dependency_edges(mainItem.node.item.key));
        diagram.destroySubItems(rightItems);
        
        diagram.rightItems = edges.map(edge => {
            const subItem = diagram.createItem(edge.to, { position: 2 });
            const connector = subItem.createConnector(mainItem, subItem);
            /*subItem*/connector.opacity = Qt.binding(() => {
                if (mainItem.highlightedMemberIndex < 0) return 1.0;
                if (edge.from_indices.includes(mainItem.highlightedMemberIndex)) return 1.0;
                return 0.2;
            });
            return subItem;
        });
    }

    function createItem(node, params) {
        return itemComponent.createObject(diagram, Object.assign({}, params, { node }));
    }
    
    function destroySubItems(subItems) {
        if (!subItems) return;

        for (let subItem of subItems) {
            subItem.destroy();        
        }
    }
    
    // Decide on the best collapsed/expanded state of items depending on the available space
    function adjustItemStateForAvailableSpace() {
        if (_adjustingItems) {
            console.log("WARNING: recursion detected in Diagram::adjustItemStateForAvailableSpace!");
            return;
        }

        _adjustingItems = true;

        const leftItems = diagram.leftItems || [];
        const rightItems = diagram.rightItems || [];

        leftItems.forEach(item => {
            item.implicitExpanded = true;
            item.implicitOnlyShowRelevantMembers = false;
        });
        let totalLeftHeight = leftItems.reduce((acc, item) => acc + item.heightAfterAnimation, 0) + Math.max(0, leftItems.length - 1) * verticalSpacing;
        
        // Show/hide non-relevant members depending on available height
        let leftTooHigh = totalLeftHeight > diagramScrollView.height - verticalPadding * 2;
        leftItems.forEach(item => {
            item.implicitOnlyShowRelevantMembers = leftTooHigh;
        });

        // Expand/collapse left items depending on available height
        totalLeftHeight = leftItems.reduce((acc, item) => acc + item.heightAfterAnimation, 0) + Math.max(0, leftItems.length - 1) * verticalSpacing;
        leftTooHigh = totalLeftHeight > diagramScrollView.height - verticalPadding * 2;
        leftItems.forEach(item => {
            item.implicitExpanded = !leftTooHigh;
        });
        
        rightItems.forEach(item => {
            item.implicitExpanded = true;
        });
        const totalRightHeight = rightItems.reduce((acc, item) => acc + item.heightAfterAnimation, 0) + Math.max(0, rightItems.length - 1) * verticalSpacing;

        // Expand/collapse right items depending on available height
        const rightTooHigh = totalRightHeight > diagramScrollView.height - verticalPadding * 2;
        rightItems.forEach(item => {
            item.implicitExpanded = !rightTooHigh;
        });

        _adjustingItems = false;
    }

    function layout() {
        if (!mainItem) return;
        if (_layouting) return;

        _layouting = true;

        const leftItems = diagram.leftItems || [];
        const rightItems = diagram.rightItems || [];
        const columnCount = 1 + (leftItems.length > 0 ? 1 : 0) + 1;
        
        // Calculate the total height of items (using the real height, not the one after animation)
        const totalLeftHeight = leftItems.reduce((acc, item) => acc + item.height, 0) + Math.max(0, leftItems.length - 1) * verticalSpacing;
        const totalRightHeight = rightItems.reduce((acc, item) => acc + item.height, 0) + Math.max(0, rightItems.length - 1) * verticalSpacing;

        const maxLeftItemWidth = leftItems.reduce((acc, item) => Math.max(acc, item.width), 0);
        const maxRightItemWidth = rightItems.reduce((acc, item) => Math.max(acc, item.width), 0);
        
        // Resize diagram
        const itemAreaWidth = horizontalPadding * 2 + mainItem.width + horizontalSpacing * (columnCount - 1) + maxLeftItemWidth + maxRightItemWidth;
        const itemAreaHeight = verticalPadding * 2 + Math.max(Math.max(totalLeftHeight, totalRightHeight), mainItem.height);
        if (itemAreaWidth > diagramScrollView.width) {
            diagram.width = itemAreaWidth;
        } else {
            diagram.width = diagramScrollView.width;
        }
        if (itemAreaHeight > diagramScrollView.height) {
            diagram.height = itemAreaHeight;
        } else {
            diagram.height = diagramScrollView.height;
        }

        // Position root item
        mainItem.x = (diagram.width - itemAreaWidth) / 2 + horizontalPadding + (leftItems.length > 0 ? maxLeftItemWidth + horizontalSpacing : 0);
        mainItem.y = diagram.height / 2 - mainItem.height / 2;
        
        // Position/resize left items
        let currentY = (diagram.height - totalLeftHeight) / 2;
        leftItems.forEach(item => {
            item.x = mainItem.x - horizontalSpacing - item.width;
            item.y = currentY;
            currentY += item.height + verticalSpacing;
        });

        // Position right items
        currentY = (diagram.height - totalRightHeight) / 2;
        rightItems.forEach(item => {
            item.x = mainItem.x + mainItem.width + horizontalSpacing;
            item.y = currentY;
            currentY += item.height + verticalSpacing;
        });
        
        // Position scrollview content (TODO: only if it was already centered)
        diagramScrollView.contentX = (diagram.width - diagramScrollView.width) / 2;
        diagramScrollView.contentY = (diagram.height - diagramScrollView.height) / 2;

        _layouting = false;
    }

    Component {
        id: itemComponent
        
        DiagramItem {
            id: diagramItem
            width: diagramItem === mainItem ? mainItemWidth : otherItemWidth
            
            onHeightChanged: {
                diagram.layout();
            }
            
            onExplicitExpandedChanged: {
                if (position === 1) return;
                
                if (_updatingExplicitExpanded) return;
                _updatingExplicitExpanded = true;

                // Ensure only 1 item has explicitExpanded set
                const otherItems = (position === 0 ? diagram.leftItems : diagram.rightItems) || [];
                otherItems.forEach(otherItem => {
                    if (otherItem !== diagramItem) {
                        if (otherItem.explicitExpanded) otherItem.explicitExpanded = undefined;
                        if (otherItem.explicitOnlyShowRelevantMembers) otherItem.explicitOnlyShowRelevantMembers = undefined;
                    }
                });
                
                _updatingExplicitExpanded = false;
            }
            
            onExplicitOnlyShowRelevantMembersChanged: {
                if (position !== 0) return;

                if (_updatingExplicitShowRelevantMembers) return;
                _updatingExplicitShowRelevantMembers = true;

                // Ensure only 1 item has explicitOnlyShowRelevantMembers set
                const otherItems = diagram.leftItems || [];
                otherItems.forEach(otherItem => {
                    if (otherItem !== diagramItem && otherItem.explicitOnlyShowRelevantMembers === false) {
                        otherItem.explicitOnlyShowRelevantMembers = undefined;
                    }
                });
                
                _updatingExplicitShowRelevantMembers = false;
            }

            onOpenClicked: {
                navigation.push(JSON.parse(vuk.get_root_node(diagramItem.node.item.key)));
            }

            onContainsMouseChanged: {
                if (containsMouse) {
                    diagram.highlightedItem = diagramItem.node.item;
                } else if (diagram.highlightedItem === diagramItem.node.item) {
                    diagram.highlightedItem = null;
                }
            }
        }
    }
}
