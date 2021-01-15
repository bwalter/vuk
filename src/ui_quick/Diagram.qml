import QtQuick 2.9
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

Rectangle {
    id: diagram
    color: "white"
    
    // Initial size = scrollview size
    width: diagramScrollView.width
    height: diagramScrollView.height
    
    property real horizontalPadding: Math.max(50, Math.min(100, diagramScrollView.width / 20))
    property real horizontalSpacing: Math.max(100, Math.min(200, diagramScrollView.width / 10))
    property real verticalSpacing: 30
    property real verticalPadding: 50
    property real mainItemWidth: Math.max(300, Math.min(800, diagramScrollView.width / 3.5))
    property real otherItemWidth: Math.max(200, Math.min(600, (diagramScrollView.width - mainItemWidth - horizontalPadding * 2 - horizontalSpacing * 2) / 2))
        
    onRootItemChanged: {
        scale = 0;
        delayedInitTimer.start();
        appearInAnimation.start();
    }
    
    readonly property int transitionDuration: 350

    property Item rootItem: null
    property bool ready: false

    ScaleAnimator {
        id: appearInAnimation
        target: diagram
        from: 0.0
        to: 1.0
        duration: 200
    }
    
    Timer {
        id: delayedInitTimer
        interval: 100
        repeat: false
        onTriggered: diagram.ready = true
    }
    
    Connections {
        target: diagramScrollView
        function onWidthChanged() { diagram.layout() }
        function onHeightChanged() { diagram.layout() }
    }
    
    function updateRoot(rootNode) {
        ready = false;
        
        // Destroy current root item and its sub items
        if (rootItem) {
            destroySubItems(rootItem.leftItems);
            destroySubItems(rootItem.rightItems);
            rootItem.destroy();
        }
        
        if (!rootNode) return;

        // Create new one
        rootItem = createItem(rootNode, { position: 1, expanded: true });
        
        // Expand it
        rootItem.expandLeft();
        rootItem.expandRight();
        
        // Layout diagram
        layout();

        vuk.currentItem = rootNode.item;
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
    
    function layout() {
        if (!rootItem) return;
        
        const leftItems = rootItem.leftItems || [];
        const rightItems = rootItem.rightItems || [];
        const columnCount = 1 + (leftItems.length > 0 ? 1 : 0) + 1;
        
        const totalLeftHeight = leftItems.reduce((acc, item) => acc + item.height, 0) + Math.max(0, leftItems.length - 1) * verticalSpacing;
        const totalRightHeight = rightItems.reduce((acc, item) => acc + item.height, 0) + Math.max(0, rightItems.length - 1) * verticalSpacing;
        
        const maxLeftItemWidth = leftItems.reduce((acc, item) => Math.max(acc, item.width), 0);
        const maxRightItemWidth = rightItems.reduce((acc, item) => Math.max(acc, item.width), 0);
        
        // Resize diagram
        const itemAreaWidth = horizontalPadding * 2 + rootItem.width + horizontalSpacing * (columnCount - 1) + maxLeftItemWidth + maxRightItemWidth;
        const itemAreaHeight = verticalPadding * 2 + Math.max(Math.max(totalLeftHeight, totalRightHeight), rootItem.height);
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
        rootItem.x = (diagram.width - itemAreaWidth) / 2 + horizontalPadding + (leftItems.length > 0 ? maxLeftItemWidth + horizontalSpacing : 0);
        rootItem.y = diagram.height / 2 - rootItem.height / 2;
        
        // Position/resize left items
        let currentY = (diagram.height - totalLeftHeight) / 2;
        leftItems.forEach(item => {
            item.x = rootItem.x - horizontalSpacing - item.width;
            item.y = currentY;
            currentY += item.height + verticalSpacing;
        });

        // Position right items
        currentY = (diagram.height - totalRightHeight) / 2;
        rightItems.forEach(item => {
            item.x = rootItem.x + rootItem.width + horizontalSpacing;
            item.y = currentY;
            currentY += item.height + verticalSpacing;
        });
        
        // Position scrollview content (TODO: only if it was already centered)
        diagramScrollView.contentX = (diagram.width - diagramScrollView.width) / 2;
        diagramScrollView.contentY = (diagram.height - diagramScrollView.height) / 2;
    }

    Component {
        id: itemComponent
        
        DiagramItem {
            id: diagramItem
            width: diagramItem === rootItem ? mainItemWidth : otherItemWidth
            
            onHeightChanged: {
                diagram.layout();
            }
            
            onOpenClicked: {
                navigation.push(JSON.parse(vuk.get_root_node(diagramItem.node.item.key)));
            }

            property var leftItems: null
            property var rightItems: null
            
            function expandLeft() {
                const edges = JSON.parse(vuk.get_dependent_edges(diagramItem.node.item.key));
                diagram.destroySubItems(leftItems);
                
                diagramItem.leftItems = edges.map(edge => {
                    const subItem = diagram.createItem(edge.to, { position: 0, expanded: false });
                    subItem.createConnector(subItem, diagramItem);
                    subItem.memberFilter = edge.from_indices;
                    return subItem;
                });
            }

            function expandRight() {
                const edges = JSON.parse(vuk.get_dependency_edges(diagramItem.node.item.key));
                diagram.destroySubItems(rightItems);
                
                diagramItem.rightItems = edges.map(edge => {
                    const subItem = diagram.createItem(edge.to, { position: 2, expanded: false });
                    subItem.createConnector(diagramItem, subItem);
                    subItem.opacity = Qt.binding(() => {
                        if (diagramItem.highlightedMember < 0) return 1.0;
                        if (edge.from_indices.includes(diagramItem.highlightedMember)) return 1.0;
                        return 0.2;
                    });
                    return subItem;
                });
            }
        }
    }
}