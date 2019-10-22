context('Counter', () => {
  beforeEach(() => {
    cy.visit('http://127.0.0.1:9999/counter/index.html');
  });

  it('works', () => {
    for (let i = 0; i < 7; i++) {
      cy.get('button')
        .contains('+')
        .click();
    }
    cy.get('div')
      .contains('7')
      .should('have.text', '- 7 + Reset');

    for (let i = 0; i < 14; i++) {
      cy.get('button')
        .contains('-')
        .click();
    }
    cy.get('div')
      .contains('-7')
      .should('have.text', '- -7 + Reset');
  });
});
