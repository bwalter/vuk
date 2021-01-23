import QtQuick 2.9
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQuick.Controls.Material 2.1
import "./style"

Label {
    anchors { left: parent.left; right: parent.right }
    wrapMode: Text.Wrap

    text: {
        if (diagram.highlightedMember) {
            // Member docu
            diagram.mainItem.node.item.name + "::<strong>" + diagram.highlightedMember.text + "</strong><br/><br/>" +
                    "<i>" + diagram.highlightedMember.docu + "</i>";
        } else if (diagram.highlightedItem) {
            // Item docu
            diagram.highlightedItem.item_type + " <strong>" + diagram.highlightedItem.name + "</strong>" + "<br/><br/>" +
                    "<i>" + diagram.highlightedItem.docu + "</i>";
        } else if (vuk.currentItem) {
            // Item docu
            vuk.currentItem.item_type + " <strong>" + vuk.currentItem.name + "</strong>" + "<br/><br/>" +
                    "<i>" + vuk.currentItem.docu + "</i>";
        } else {
            // Nothing
            "";
        }
    }
}
