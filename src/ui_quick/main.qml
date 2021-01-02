import QtQuick 2.9
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQuick.Controls.Material 2.1
import Vuk 1.0

ApplicationWindow {
    id: window
    visible: true
    width: 800
    height: 600
    
    property bool implicitAnimationsEnabled: false
    
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
                text: qsTr("Open &File...")
                onTriggered: {
                    const dialog = openFileDialogComponent.createObject(null, {});
                    dialog.open();
                    dialog.closed.connect(() => dialog.destroy());
                }
            }
            Action {
                text: qsTr("Open Folder...")
                onTriggered: {
                    const dialog = openFileDialogComponent.createObject(null, { selectFolder: true });
                    dialog.open();
                    dialog.closed.connect(() => dialog.destroy());
                }
            }
            MenuSeparator { }
            Action { text: qsTr("&Quit") }
        }

        Menu {
            title: qsTr("&View")
        }

        Menu {
            title: qsTr("&Help")
            Action { text: qsTr("&About") }
        }
    }
    
    header: ToolBar {
        RowLayout {
            anchors.fill: parent
            ToolButton {
                text: qsTr("Open")
                onClicked: stack.pop()
            }
            ToolButton {
                text: qsTr("Refresh")
                onClicked: stack.pop()
            }
            Label {
                text: "Title"
                elide: Label.ElideRight
                horizontalAlignment: Qt.AlignHCenter
                verticalAlignment: Qt.AlignVCenter
                Layout.fillWidth: true
            }
            ToolButton {
                text: qsTr("⋮")
                onClicked: menu.open()
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
            selectionState = JSON.parse(selection);
        }
        
        onError: {
            console.log("QUICK says: there is an error:", error);
        }
    }
    
    Colors {
        id: colors
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