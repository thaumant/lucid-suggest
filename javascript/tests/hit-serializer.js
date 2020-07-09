module.exports = {
    test(value) {
        return value instanceof Array && value[0]?.constructor?.name == "Hit"
    },
    print(hits, serialize) {
        return serialize(hits.map(hit => ({
            id:     hit.record.id,
            rating: hit.record.rating,
            title:  hit.title,
        })))
    }
}
