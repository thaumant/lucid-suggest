
exports.Stemmer = function Stemmer() {
    this.setCurrent = function(word) {
        this.current = word;
    };

    this.getCurrent = function() {
        return this.current;
    };

    this.stem = function() {
        return true;
    }
}
