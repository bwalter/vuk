import QtQuick 2.9
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQuick.Controls.Material 2.1
import Qt.labs.settings 1.0
import "./style"
import Vuk 1.0

ApplicationWindow {
    id: window
    visible: true
    width: 1400
    height: 900
    
    property bool implicitAnimationsEnabled: false
    property string latestUrl: ""
    
    Material.theme: Material.Light
    Material.accent: Material.Blue
    
    Actions {
        id: actions
    }
    
    menuBar: MenuBar {
         Menu {
            title: "&File"
            MenuItem { action: actions.openFolder }
            MenuItem { action: actions.refresh }
            MenuSeparator {}
            MenuItem { action: actions.quit }
        }

        Menu {
            title: "&View"
            MenuItem { action: actions.decreaseFontSize }
            MenuItem { action: actions.increaseFontSize }
        }

        Menu {
            title: "&Go"
            MenuItem { action: actions.goBackward }
            MenuItem { action: actions.goForward }
        }

        Menu {
            title: "&Help"
            //MenuItem { actions: actions.about; text: "&About" }
        }
    }
    
    header: ToolBar {
        RowLayout {
            id: toolbarRowLayout

            CustomToolButton { action: actions.openFolder }
            CustomToolButton { action: actions.refresh }

            Item { width: 30 }

            CustomToolButton { action: actions.goBackward }
            CustomToolButton { action: actions.goForward }

            Item { width: 30 }

            CustomToolButton {
                action: actions.decreaseFontSize
                text: "Smaller"
            }
            CustomToolButton {
                action: actions.increaseFontSize
                text: "Larger"
            }

            Item { Layout.fillWidth: true }
        }
    }
    
    Settings {
        property alias windowWidth: window.width
        property alias windowHeight: window.height
        property alias latestUrl: window.latestUrl
        //property alias fontFactor: Style.fontFactor
        property alias mainSplitViewState: mainSplitView.state
    }
    
    OpenFileDialog {
        id: openFileDialog
        selectFolder: true

        onUrlSelected: {
            navigation.clear();
            latestUrl = url;
            vuk.open(url);
        }
    }
    
    Navigation {
        id: navigation
        onCurrentChanged: {
            selectionView.setCurrent(current);
        }
    }

    Vuk {
        id: vuk
        Component.onCompleted: {
            if (latestUrl.length > 0) {
                vuk.open(latestUrl);
            }
        }
        
        property var selectionState: {}
        property var currentItem: null
        
        onSelectionChanged: {
            navigation.clear();
            selectionState = JSON.parse(selection);
        }
        
        onError: {
            console.log("QUICK says: there is an error:", error);
        }
    }
    
    SplitView {
        id: mainSplitView
        anchors.fill: parent
        orientation: Qt.Horizontal
        
        property var state: null
        
        Component.onCompleted: {
            if (state !== null) {
                console.log("Restoring SplitView state...");
                restoreState(state);
            }
        }
        
        onResizingChanged: {
            state = saveState();
        }
        
        SplitView {
            SplitView.preferredWidth: window.width / 4
            SplitView.maximumWidth: window.width / 2

            orientation: Qt.Vertical
            
            Selection {
                id: selectionView
                SplitView.fillHeight: true
                SplitView.fillWidth: true
            }

            ScrollView {
                SplitView.preferredHeight: window.height / 3.2
                SplitView.fillWidth: true

                contentWidth: parent.width
                contentHeight: docu.height
                clip: true

                Documentation {
                    id: docu
                }
            }
        }
        
        Item {
            SplitView.fillHeight: true 
            SplitView.fillWidth: true 

            Flickable {
                id: diagramScrollView
                anchors.fill: parent

                //ScrollBar.vertical.policy: ScrollBar.AutoHide
                //ScrollBar.horizontal.policy: ScrollBar.AutoHide
                boundsBehavior: Flickable.StopAtBounds 
                clip: true
                
                contentWidth: diagram.width
                contentHeight: diagram.height
                
                Rectangle {
                    anchors.fill: parent
                    color: "white"
                }
                
                Diagram {
                    id: diagram
                    transformOrigin: Item.Center
                    //scale: Math.max(0.5, Math.min(4, zoom))

                    property real zoom: 1.0
                    property real zoomStep: 0.1
                }
            }
            
            //MouseArea {
            //    anchors.fill: parent
            //    acceptedButtons: Qt.NoButton

            //    onWheel: {
            //        if (wheel.angleDelta.y > 0)
            //            diagram.zoom = Number((diagram.zoom + diagram.zoomStep).toFixed(1))
            //        else
            //            if (diagram.zoom > 0) diagram.zoom = Number((diagram.zoom - diagram.zoomStep).toFixed(1))

            //        wheel.accepted = true
            //    }
            //}
        }
    }
}
