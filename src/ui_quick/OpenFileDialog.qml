import QtQuick 2.9
import QtQuick.Dialogs 1.0

FileDialog {
    id: fileDialog
    title: "Select file or directory"
    folder: shortcuts.home
    
    signal urlSelected(string url);
    signal closed();
    
    Component.onCompleted: console.log("OpenFileDialog CREATED")

    onAccepted: {
        console.log("You chose: " + fileDialog.fileUrls);
        if (fileDialog.fileUrls.length > 0) {
            urlSelected(fileDialog.fileUrls[0]);
        }
    }

    onRejected: {
    }
}