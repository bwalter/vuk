import QtQuick 2.9
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQuick.Controls.Material 2.1
import Vuk 1.0

Label {
    anchors { left: parent.left; right: parent.right }
    wrapMode: Text.Wrap
    text: {
        if (!vuk.currentItem) return "";

        vuk.currentItem.item_type + " <strong>" + vuk.currentItem.name + "</strong>" + "<br/><br/>" +
                "<i>" + vuk.currentItem.docu + "</i>";
    }
}