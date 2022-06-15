<template fluid>
  <div>
    <v-alert type="success" v-model="alertFinishedEncryption">
      Document encrypted
    </v-alert>
    <v-dialog v-model="dialogSelectionWindowMenu" max-width="500">
      <v-card>
        <v-card-title>Add selected text for encryption</v-card-title>
        <v-select
          :items="roles"
          label="roles"
          single-line
          @change="updateSelectedRole"
        ></v-select>
        <v-card-text>{{ currentFragmentDraft?.content }}</v-card-text>
        <v-card-actions>
          <v-btn color="red" @click="closeSelectionWindowMenu">Cancel</v-btn>
          <v-btn color="blue darken-1" @click="addForEncryption">Add</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <v-card elevation="4" class="mx-auto my-12" max-width="574">
      <v-card-title>This is Lorem Ipsum Document</v-card-title>
      <v-card-text>
        <textarea
          name=""
          id=""
          cols="100"
          rows="10"
          @select="onSelectedText"
          readonly
          v-model="textExample"
        >
        </textarea>
      </v-card-text>
      <v-card-actions>
        <v-btn color="deep-purple lighten-2" @click="finishEncryption">
          Finish Encrypting
        </v-btn>
      </v-card-actions>
    </v-card>
  </div>
</template>

<script>
//import SelectionWindowMenu from "../dialogs/SelectionWindowMenu.vue";
import Document from "../types/document";
import { TechnoKing, Manager, Monkey } from "../types/role";
import TextFragment from "../types/textFragment";
import rolesMap from "../types/textFragment";

const textExample = `
    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
      tempor incididunt ut labore et dolore magna aliqua. Id interdum velit
      laoreet id donec ultrices tincidunt. Luctus venenatis lectus magna
      fringilla urna porttitor rhoncus. Mi proin sed libero enim sed faucibus
      turpis in. Sit amet nulla facilisi morbi tempus iaculis. Habitasse platea
      dictumst quisque sagittis. In vitae turpis massa sed elementum tempus.
      Arcu dictum varius duis at consectetur lorem donec massa sapien. Arcu dui
      vivamus arcu felis bibendum ut tristique et. Mauris nunc congue nisi vitae
      suscipit. Augue mauris augue neque gravida in fermentum et
      sollicitudin.
    `;

export default {
  // components: { SelectionWindowMenu },
  data: () => ({
    alertFinishedEncryption: false,
    dialogSelectionWindowMenu: false,
    selectedText: null,
    textSelectedForEncryption: "",
    currentFragmentDraft: null,
    roles: [TechnoKing.name, Manager.name, Monkey.name],
    textExample,
    currentDocument: new Document(textExample),
    selectedRoleName: null,
  }),
  methods: {
    onSelectedText(event) {
      let textArea = event.target;
      let selectedText = textArea.value.slice(
        textArea.selectionStart,
        textArea.selectionEnd
      );

      this.currentFragmentDraft = new TextFragment(
        selectedText,
        textArea.selectionStart,
        textArea.selectionEnd
      );
      this.displaySelectionWindowMenu();
    },
    displaySelectionWindowMenu() {
      this.dialogSelectionWindowMenu = true;
    },
    closeSelectionWindowMenu() {
      this.dialogSelectionWindowMenu = false;
    },
    updateSelectedRole(roleName) {
      this.selectedRoleName = roleName;
      console.log(roleName);
    },
    addForEncryption() {
      let roleName = this.selectedRoleName;
      this.currentFragmentDraft.role = rolesMap[roleName];
      this.currentDocument.addFragment(this.currentFragmentDraft);
      this.currentFragmentDraft = null;
      console.log(roleName);
      this.closeSelectionWindowMenu();
    },
    finishEncryption() {
      this.alertFinishedEncryption = true;
    },
  },
};
</script>
<style>
textarea {
  border: none;
  outline: none;
}
</style>
