import { readFileSync } from "fs";

console.log("Its start here");

const jsonString = readFileSync("./others/signatures.json", "utf8");
let json = JSON.parse(jsonString);

console.log(json);

// SMS function in the study videos
function SMS(): void {
    const lib = require('messagemedia-messages-sdk');

    lib.Configuration.basicAuthUserName = "YOUR_BASIC_API_KEY";
    lib.Configuration.basicAuthPassword = "YOUR_BASIC_SECRET_KEY";

    var controller = lib.MessagesController;

    let body = new lib.SendMessagesRequest();

    body.messages = [];

    body.messages[0] = new lib.Message();

    body.messages[0].content = 'save me...';
    body.messages[0].content.destinationNumber ='DQ5zl4ighWwiag7y+cWFQg==';

    controller.sendMessages(body, function(error, response, context) {
        if (error) {
            console.log(error)
        } else {
            console.log(response);
        }
    });
}
