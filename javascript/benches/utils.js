
function first2digits(num) {
    let scale = 1
    while (num >= 100) {
        scale *= 10
        num   /= 10
    }
    return Math.round(num) * scale
}

exports.first2digits = first2digits
