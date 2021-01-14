import QtQuick 2.9
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQuick.Controls.Material 2.1
import Vuk 1.0

ColumnLayout {
    Rectangle {
        Layout.fillWidth: true
        height: childrenRect.height + 2 * 6

        RowLayout {
            id: filterLayout
            anchors { left: parent.left; right: parent.right; verticalCenter: parent.verticalCenter }
            anchors { leftMargin: 8; rightMargin: 8 }
            
            Label {
                text: "Filter:"
            }

            TextField {
                id: filterField
                Layout.fillWidth: true
                selectByMouse: true
            }
        }
    }

    ListView {
        id: selectionView
        Layout.fillWidth: true
        Layout.fillHeight: true

        model: vuk.selectionState && vuk.selectionState.items
        clip: true
        
        onModelChanged: currentIndex = -1

        delegate: ItemDelegate {
            id: delegate
            width: parent ? parent.width : 0
            highlighted: ListView.isCurrentItem
            
            contentItem: Row {
                ItemSymbol {
                    anchors.verticalCenter: parent.verticalCenter
                    itemType: modelData.item_type
                }

                Label {
                    anchors.verticalCenter: parent.verticalCenter
                    leftPadding: 8
                    rightPadding: 8
                    text: delegate.text
                    font.pointSize: 10
                    //color: delegate.highlighted ? "white" : Qt.darker(colors.colorOfItemType(modelData.item_type))
                    //color: delegate.highlighted ? "white" : "black"
                    color: "black"
                    elide: Text.ElideRight
                    verticalAlignment: Text.AlignVCenter
                }
            }

            text: {
                const pkgColor = highlighted ? "white" : "gray";
                
                modelData.name + "<font color=\"" + pkgColor + "\" size=\"2\"> (" + modelData.pkg_path + ")</font>"
            }
            
            states: State {
                name: "hidden"
                when: !modelData.name.toLowerCase().includes(filterField.text.toLowerCase())
                PropertyChanges { target: delegate; height: 0; visible: false }
            }

            onClicked: {
                selectionView.currentIndex = index;
                diagram.updateRoot(JSON.parse(vuk.get_root_node(modelData.key)));
            }
        }
    }
}
