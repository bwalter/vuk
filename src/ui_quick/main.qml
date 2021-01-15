import QtQuick 2.9
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQuick.Controls.Material 2.1
import Vuk 1.0

ApplicationWindow {
    id: window
    visible: true
    width: 1400
    height: 900
    
    property bool implicitAnimationsEnabled: false
    property string latestUrl: ""
    readonly property Item style: style
    
    Component.onCompleted: {
        Qt.application.name = "Vuk";
        Qt.application.organization = "bwa";
        Qt.application.domain = "com.bwa.vuk"
    } 
    
    Material.theme: Material.Light
    Material.accent: Material.Blue
    
    menuBar: MenuBar {
         Menu {
            title: qsTr("&File")
            Action {
                id: openFolderAction
                text: qsTr("Open Folder...")
                onTriggered: {
                    const dialog = openFileDialogComponent.createObject(null, { selectFolder: true });
                    dialog.open();
                    dialog.urlSelected.connect(url => {
                        navigation.clear();
                        latestUrl = url;
                        vuk.open(url);
                    });
                    dialog.closed.connect(() => dialog.destroy());
                }
            }
            MenuSeparator { }
            Action { text: qsTr("&Quit") }
        }

        Menu {
            title: qsTr("&View")
            Action {
                id: recreaseFontSizeAction
                text: qsTr("Decrease font size")
                onTriggered: {
                }
            }
            Action {
                id: increaseFontSizeAction
                text: qsTr("Increase font size")
                onTriggered: {
                }
            }
        }

        Menu {
            title: qsTr("&Go")
        }

        Menu {
            title: qsTr("&Help")
            Action { text: qsTr("&About") }
        }
    }
    
    header: ToolBar {
        RowLayout {
            id: toolbarRowLayout

            CustomToolButton {
                text: "Open"
                iconCode: "\uED25"
                onClicked: openFolderAction.triggered()
            }
            CustomToolButton {
                text: "Refresh"
                iconCode: "\uE72C"
                enabled: latestUrl !== ""
                onClicked: {
                    navigation.clear();
                    vuk.open(latestUrl)
                }
            }
            Item {
                width: 30
            }
            CustomToolButton {
                iconCode: "\uE72B"
                text: "Back"
                enabled: navigation.canGoBackward
                onClicked: navigation.previous()
            }
            CustomToolButton {
                iconCode: "\uE72A"
                text: "Forward"
                enabled: navigation.canGoForward
                onClicked: navigation.next()
            }
            Item {
                width: 30
            }
            CustomToolButton {
                iconCode: "\uE8E7"
                text: "Smaller"
                enabled: style.canDecreaseFontSize
                onClicked: style.decreaseFontSize()
            }
            CustomToolButton {
                iconCode: "\uE8E8"
                text: "Larger"
                enabled: style.canIncreaseFontSize
                onClicked: style.increaseFontSize()
            }
            Item {
                width: 30
            }
            CustomToolButton {
                text: "Collapse all"
                iconCode: "\uF166"
                onClicked: {}
            }
            CustomToolButton {
                text: "Expand all"
                iconCode: "\uF164"
                onClicked: {}
            }
            Item {
                Layout.fillWidth: true
            }
        }
    }
    
    Component {
        id: openFileDialogComponent
        OpenFileDialog {}
    }
    
    Vuk {
        id: vuk
        //Component.onCompleted: init()
        
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
    
    Style {
        id: style
    }
    
    Navigation {
        id: navigation
        function onCurrentChanged(current) {
            selection.setCurrent(current);
        }
    }

    SplitView {
        anchors.fill: parent
        orientation: Qt.Horizontal
        
        SplitView {
            SplitView.preferredWidth: window.width / 4
            SplitView.maximumWidth: window.width / 2

            orientation: Qt.Vertical
            
            Selection {
                SplitView.fillHeight: true
                SplitView.fillWidth: true
            }

            ScrollView {
                SplitView.preferredHeight: 300
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
