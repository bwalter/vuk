import QtQuick 2.9
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQuick.Controls.Material 2.1
import "./style"

ColumnLayout {
    id: selection

    function setCurrent(node) {
        if (!node) {
            selectionView.currentIndex = -1;
            return;
        }

        const index = selectionView.model.findIndex(selectionItem => selectionItem.key === node.item.key);
        selectionView.currentIndex = index;
    }

    Rectangle {
        Layout.fillWidth: true
        height: childrenRect.height + 6

        RowLayout {
            id: filterLayout
            anchors { left: parent.left; right: parent.right; verticalCenter: parent.verticalCenter }
            anchors { leftMargin: 8; rightMargin: 8 }
            
            Label {
                text: "Filter:"
                font.family: Style.mainFontFamily
            }

            TextField {
                id: filterField
                Layout.fillWidth: true
                font.family: Style.mainFontFamily
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

        section.property: "model.modelData.pkg_path"
        //section.labelPositioning: ViewSection.InlineLabels
        section.labelPositioning: ViewSection.CurrentLabelAtStart
        section.delegate: Rectangle {
            width: selectionView.width
            height: 30
            color: "lightGray"

            Component.onCompleted: console.log("SECTION created:", section)

            Label {
                anchors.centerIn: parent
                text: section
                color: "black"
            }
        }
        
        onModelChanged: currentIndex = -1

        delegate: ItemDelegate {
            id: delegate
            width: parent ? parent.width : 0
            highlighted: ListView.isCurrentItem
            font.family: Style.mainFontFamily
            font.pointSize: Style.listViewItemFontSize

            contentItem: Row {
                ItemSymbol {
                    anchors.verticalCenter: parent.verticalCenter
                    itemType: modelData.item_type
                    color: Qt.lighter(Style.colorOfItemType(itemType))
                }

                Label {
                    anchors.verticalCenter: parent.verticalCenter
                    leftPadding: 4
                    rightPadding: 4
                    text: delegate.text
                    font: delegate.font
                    color: delegate.highlighted ? Style.listViewItemHighlightedTextColor : Style.listViewItemTextColor
                    elide: Text.ElideRight
                    verticalAlignment: Text.AlignVCenter
                }
            }
            
            background: Rectangle {
                implicitHeight: contentItem.implicitHeight
                color: delegate.highlighted ? Style.listViewItemHighlightColor : Style.listViewItemBackgroundColor
            }

            text: {
                const pkgColor = highlighted ? Style.listViewItemHighlightedAlternativeTextColor
                    : Style.listViewItemAlternativeTextColor
                
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
