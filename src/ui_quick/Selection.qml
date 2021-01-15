import QtQuick 2.9
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQuick.Controls.Material 2.1
import Vuk 1.0

ColumnLayout {
    id: selection

    function setCurrent(node) {
        const index = selectionView.model.findIndex(selectionItem => selectionItem.key === node.item.key);
        selectionView.currentIndex = index;
    }

    Rectangle {
        Layout.fillWidth: true
        height: childrenRect.height + 2 * 6

        RowLayout {
            id: filterLayout
            anchors { left: parent.left; right: parent.right; verticalCenter: parent.verticalCenter }
            anchors { leftMargin: 8; rightMargin: 8 }
            
            Label {
                text: "Filter:"
                font.family: window.style.mainFontFamily
            }

            TextField {
                id: filterField
                Layout.fillWidth: true
                font.family: window.style.mainFontFamily
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
            font.family: style.mainFontFamily
            font.pointSize: style.listViewItemFontSize

            readonly property Item style: window.style

            contentItem: Row {
                ItemSymbol {
                    anchors.verticalCenter: parent.verticalCenter
                    itemType: modelData.item_type
                    color: Qt.lighter(style.colorOfItemType(itemType))
                }

                Label {
                    anchors.verticalCenter: parent.verticalCenter
                    leftPadding: 8
                    rightPadding: 8
                    text: delegate.text
                    font: delegate.font
                    color: delegate.highlighted ? delegate.style.listViewItemHighlightedTextColor : delegate.style.listViewItemTextColor
                    elide: Text.ElideRight
                    verticalAlignment: Text.AlignVCenter
                }
            }
            
            background: Rectangle {
                implicitHeight: contentItem.implicitHeight
                color: delegate.highlighted ? delegate.style.listViewItemHighlightColor : delegate.style.listViewItemBackgroundColor
            }

            text: {
                const pkgColor = highlighted ? delegate.style.listViewItemHighlightedAlternativeTextColor
                    : delegate.style.listViewItemAlternativeTextColor
                
                modelData.name + "<font color=\"" + pkgColor + "\" size=\"2\"> (" + modelData.pkg_path + ")</font>"
            }
            
            states: State {
                name: "hidden"
                when: !modelData.name.toLowerCase().includes(filterField.text.toLowerCase())
                PropertyChanges { target: delegate; height: 0; visible: false }
            }

            onClicked: {
                selectionView.currentIndex = index;
                navigation.push(JSON.parse(vuk.get_root_node(modelData.key)));
            }
        }
        
        ScrollIndicator.vertical: ScrollIndicator {}
    }
}
