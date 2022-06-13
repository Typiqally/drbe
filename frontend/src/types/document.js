import { SuperRole } from "./role";
import TextFragment from "./textFragment";

export default class Document {
  constructor(content) {
    this.fragments = [];
    this.fragments.push(
      new TextFragment(content, 0, content.length - 1, SuperRole)
    );
    console.log(this);
  }
  addFragment(newFragment) {
    this.fragments.forEach((fragment, index) => {
      //newFragment is inside of an existing one
      if (
        fragment.fragmentStart < newFragment.fragmentStart &&
        fragment.fragmentEnd > newFragment.fragmentEnd
      ) {
        let leftFragment = fragment.slice(
          fragment.fragmentStart,
          newFragment.fragmentStart
        );
        let rightFragment = fragment.slice(
          newFragment.fragmentEnd + 1,
          fragment.fragmentEnd
        );
        //deleting element on index and add in this scope 3 elements
        this.fragments.splice(
          index,
          1,
          leftFragment,
          newFragment,
          rightFragment
        );
        return;
      }

      //newFragment shares the left side
      if (
        fragment.fragmentStart == newFragment.fragmentStart &&
        fragment.fragmentEnd > newFragment.fragmentEnd
      ) {
        let rightFragment = fragment.slice(
          newFragment.fragmentEnd + 1,
          fragment.fragmentEnd
        );
        //deleting element on index add add in this scope 2 elements
        this.fragments.splice(index, 1, newFragment, rightFragment);
        return;
      }

      if (
        fragment.fragmentEnd == newFragment.fragmentEnd &&
        fragment.fragmentStart < newFragment.fragmentStart
      ) {
        let leftFragment = fragment.slice(
          fragment.fragmentStart,
          newFragment.fragmentStart
        );
        //deleting element on index add add in this scope 2 elements
        this.fragments.splice(index, 1, leftFragment, newFragment);
        return;
      }
      if (
        fragment.fragmentEnd == newFragment.fragmentEnd &&
        fragment.fragmentStart == newFragment.fragmentStart
      ) {
        this.fragments.splice(index, 1, newFragment);
        return;
      }

      //newFragment spans two existing fragments
      if (
        fragment.fragmentStart < newFragment.fragmentStart &&
        fragment.fragmentEnd < newFragment.fragmentEnd
      ) {
        let leftFragment = fragment.slice(
          fragment.fragmentStart,
          newFragment.fragmentStart
        );
        let rightFragment = this.fragments[index + 1];
        rightFragment = rightFragment.slice(
          newFragment.fragmentEnd,
          rightFragment.fragmentEnd
        );
        this.fragments.splice(
          index,
          1,
          leftFragment,
          newFragment,
          rightFragment
        );
        return;
      }
    });
  }
}
