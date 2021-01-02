import QtQuick 2.9

QtObject {
    function colorOfItemType(itemType) {
        switch (itemType) {
            case "Interface": return "#FFCAB0";
            case "Enum": return "#B0D9FF";
            case "Struct": return "#F9EC82";
            default: return "#EEEEEE";
        }
    }
}