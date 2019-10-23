context('Form', () => {
  beforeEach(() => {
    cy.visit('http://127.0.0.1:9999/form/index.html');
  });

  it('works', () => {
    const assertHasText = string =>
      cy
        .get('body')
        .contains(string)
        .should('exist');

    assertHasText('username: "",');

    cy.get('#username').type('Ferris');

    assertHasText('username: "Ferris",');

    cy.get('button')
      .contains('Clear')
      .click();

    assertHasText('username: "",');

    assertHasText('plan: "C3"');

    cy.get('select').select('D4');

    assertHasText('plan: "D4"');

    cy.get('button')
      .contains('Submit')
      .should('be.disabled');

    cy.get('button')
      .contains('Agree')
      .click();

    cy.get('button')
      .contains('Submit')
      .should('be.enabled');

    const stub = cy.stub();

    cy.on('window:alert', stub);

    cy.get('#username').type('Ferris');
    cy.get('#password').type('hunter2');

    cy.get('button')
      .contains('Submit')
      .click()
      .wait(1200)
      .then(() => {
        expect(stub.getCall(0)).to.be.calledWith(`Submitted: Form {
    username: "Ferris",
    password: "hunter2",
    accept: true,
    plan: "D4",
    is_submitting: false,
}`);
      });
  });
});
