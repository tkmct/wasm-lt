const N_LINE = '\n'

class Parser {
  constructor(source) {
    this.pos = 0;
    this.input = source;
  }

  parse() {
    let result = '';

    while (!this.eof()) {
      result += this.parseLine()
      result += N_LINE;
      if (this.currentChar() === '\n') {
        this.consumeChar()
      }
    }

    return result;
  }

  parseLine() {
    // Parse title
    if (this.currentChar() === '#') {
      const sharps = this.consumeWhile(c => c === '#')
      if (this.currentChar() !== ' ') {
        return sharps+this.consumeWhile(c =>  c !== '\n');
      }
      // Consume white space
      this.consumeChar();

      const text = this.consumeWhile(c => c !== '\n' );
      return '<h' + sharps.length + '>' + text + '</h' + sharps.length + '>'
    }

    // Parse list item
    if (( this.currentChar() === '-' || this.currentChar() === '*' ) && this.nextChar() === ' ') {
      // consume bullet and white space
      this.consumeChar()
      this.consumeChar()

      const text = this.consumeWhile(c => c !== '\n' );
      return '<li>' + text + '</li>'
    }

    // Parse border
    if (this.currentChar() === '*') {
      const b = this.consumeWhile(c => c === '*')
      if (b.length >= 3 && this.currentChar() === '\n') {
        return '<hr />'
      }
    }


    return '<p>' + this.consumeWhile(c => c !== '\n' ) + '</p>'
  }

  currentChar() {
    return this.input[this.pos];
  }

  nextChar () {
    return this.input[this.pos+1];
  }

  consumeChar() {
    return this.input[this.pos++];
  }

  eof() {
    return this.pos >= this.input.length
  }

  consumeWhile(cond) {
    let result = '';

    while (!this.eof() && cond(this.currentChar())) {
      result += this.consumeChar()
    }

    return result
  }
}


const btn = document.getElementById('parse-btn-js')
const previewArea = document.getElementById('preview')
btn.addEventListener('click', () => {
  const input = document.getElementById('markdown-text').value
  const parser = new Parser(input)
  const result = parser.parse()

  previewArea.innerHTML = result
})

