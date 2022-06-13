export default class Role {
  name;
  publicKey;

  constructor(name, publicKey = null) {
    this.name = name;
    this.publicKey = publicKey;
  }

  get text() {
    return this.name;
  }
}

export const SuperRole = new Role("SuperRole");
export const TechnoKing = new Role("TechnoKing", "#1231234");
export const Manager = new Role("Manager", "#1231234");
export const Monkey = new Role("Monkey", "#1231234");

export const rolesMap = { TechnoKing, Manager, Monkey };
