const request = require("request");

url = "https://discord.com/api/users/@me"

json = {
    "name": "High Five",
    "type": 2
}

request({
    url: url,
    method: "GET",
    json: true,
    headers: {
        "Authorization": "Bot ODk3MzU1NDQ5OTg3MzcxMDA5.YWUdYw.DMkChY6IZ6FAKn2dAURtB-WHYoU"
    },
  }, function(error, response, body) {
      if (!error && response.statusCode == 200) {
          console.log(body) 
          console.log(body[5].permission_overwrites)
      } else {
        console.log(body)
      }
  }); 



