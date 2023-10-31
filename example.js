console.log("Hello","run.js!");
console.error("Boom!");

const path = '/Users/Z009TZW/Documents/my_files/my-own-js-runtime/log.txt'
await runjs.createFile(path);
await runjs.writeFile(path,"I am a log file");
try {
    const contents = await runjs.readFile(path);
    console.log("From File",contents);
} catch(err) {
    console.error("Unable to read file",path,err)
}
console.log("Removing file",path);
await runjs.removeFile(path);
console.log("File removed");

const content = await runjs.fetch(
  "https://example.com/",
);
console.log("Content from fetch", content);