const $ = Cypress.$;

const withText = string => $xs => $xs.filter((_, x) => $(x).text() === string);

context('Counters', () => {
  beforeEach(() => {
    cy.visit('http://127.0.0.1:9999/counters/index.html');
  });

  it('works', () => {
    cy.get('button')
      .contains('+')
      .should('not.exist');

    cy.get('button')
      .contains('Append')
      .click();

    cy.get('button')
      .contains('+')
      .should('have.length', 1);

    for (let i = 0; i < 5; i++) {
      cy.get('button')
        .contains('Append')
        .click();
    }

    cy.get('div')
      .then(withText('- 0 + Reset Remove'))
      .should('have.length', 6);

    for (let i = 0; i < 7; i++) {
      cy.get('button')
        .then(withText('+'))
        .eq(2)
        .click();
    }

    cy.get('div')
      .then(withText('- 7 + Reset Remove'))
      .should('have.length', 1);

    for (let i = 0; i < 10; i++) {
      cy.get('button')
        .then(withText('-'))
        .eq(3)
        .click();
    }

    cy.get('div')
      .then(withText('- -10 + Reset Remove'))
      .should('have.length', 1);
  });
});
