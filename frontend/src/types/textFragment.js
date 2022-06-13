export default class TextFragment {
  constructor(content, fragmentStart, fragmentEnd, role = null) {
    this.content = content;
    this.role = role;
    this.fragmentStart = fragmentStart;
    this.fragmentEnd = fragmentEnd;
  }
  slice(start, end) {
    //getting start and end relative to the whole document, recalculating to this particular fragment
    start = start - this.fragmentStart;
    end = end - this.fragmentStart;
    let newContent = this.content.slice(start, end);
    let newFragment = new TextFragment(
      newContent,
      this.fragmentStart,
      this.fragmentEnd,
      this.role
    );
    return newFragment;
  }
}
