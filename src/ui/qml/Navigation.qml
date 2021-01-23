import QtQuick 2.9

QtObject {
    property var nodes: []
    property int position: -1
    
    readonly property bool canGoBackward: position >= 1
    readonly property bool canGoForward: position < nodes.length - 1
    
    signal currentChanged(var current);

    function clear() {
        nodes = [];
        position = -1;

        diagram.setMainNode(null);
        //selection.setCurrent(null);
        currentChanged(null);
    }
    
    function push(node) {
        nodes.push(node);
        position = nodes.length - 1;
        
        diagram.setMainNode(node);
        currentChanged(node);
    }
    
    function previous() {
        if (position <= 0) return;
        --position;

        const node = nodes[position];
        diagram.setMainNode(node);
        currentChanged(node);
    }

    function next() {
        if (position >= nodes.length - 1) return;
        ++position;

        const node = nodes[position];
        diagram.setMainNode(node);
        currentChanged(node);
    }
}
