console.log("Hello","run.js!");
console.error("Boom!");

const path = '/Users/Z009TZW/Documents/my_files/my-own-js-runtime/log.txt'
runjs.createFile(path);
runjs.writeFile(path,"I am a log file");
try {
    const contents = runjs.readFile(path);
    console.log("Read from a file",contents);
} catch(err) {
    console.error("Unable to read file",path,err)
}

runjs.writeFile(path,"I can write to a file.");
const contents = runjs.readFile(path);
console.log("Read from a file",path,"contents:",contents);
console.log("Removing file",path);
runjs.removeFile(path);
console.log("File removed");