import QtQuick 2.9
import QtQuick.Controls 2.15
import "./style"

Item {
    readonly property Action openFolder: Action {
        text: "Open Folder..."
        property string iconCode: "\uED25"
        onTriggered: {
            openFileDialog.open();
        }
    }

    readonly property Action refresh: Action {
        text: "Refresh"
        enabled: latestUrl !== ""
        property string iconCode: "\uE72C"
        onTriggered: {
            navigation.clear();
            vuk.open(latestUrl)
        }
    }

    readonly property Action quit: Action {
        text: "&Quit"
        onTriggered: Qt.app.quit()
    }

    readonly property Action decreaseFontSize: Action {
        property string iconCode: "\uE8E7"
        text: "Decrease font size"
        enabled: Style.canDecreaseFontSize
        onTriggered: Style.decreaseFontSize()
    }

    readonly property Action increaseFontSize: Action {
        text: "Increase font size"
        property string iconCode: "\uE8E8"
        enabled: Style.canIncreaseFontSize
        onTriggered: Style.increaseFontSize()
    }

    readonly property Action goBackward: Action {
        text: "Back"
        property string iconCode: "\uE72B"
        enabled: navigation.canGoBackward
        onTriggered: navigation.previous()
    }

    readonly property Action goForward: Action {
        text: "Forward"
        property string iconCode: "\uE72A"
        enabled: navigation.canGoForward
        onTriggered: navigation.next()
    }
}
