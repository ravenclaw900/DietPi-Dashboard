let toml = require('@ltd/j-toml')

module.exports.readVersion = function (contents) {
    return toml.parse(contents).package.version;
}

module.exports.writeVersion = function (contents, version) {
    const obj = toml.parse(contents)
    obj.package.version = version
    return toml.stringify(obj)
}