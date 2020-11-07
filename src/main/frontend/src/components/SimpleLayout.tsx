import React from 'react';
import { ArrowForward } from '@material-ui/icons';
import { Container, Row, Col, Nav, Navbar, } from 'react-bootstrap';

import transparent from "../img/innexgo_transparent_icon.png"
import innexgo_logo from '../img/innexgo_transparent_icon.png';

class SimpleLayout extends React.Component {
  render() {
    return (
      <Container fluid style={{ height: "100vh" }}>
        <Row className="h-100">
          <Col md="auto" className="px-3 py-3" style={{ backgroundColor: '#990000ff', }}>
            <Row md={4} className="my-2 text-light">
              <Col >
                <img src={transparent} alt="Innexgo Logo" />
              </Col>
              <Col>
                <h4>Attendance simplified.</h4>
              </Col>
            </Row>
            <a href="/" className="text-light">
              <ArrowForward />Log In
             </a>
            <br />
            <a href="/instructions" className="text-light">
              <ArrowForward />Instructions
            </a>
            <br />
            <a href="/register" className="text-light">
              <ArrowForward />Register
            </a>
            <br />
            <a href="https://hours.innexgo.com" className="text-light">
              <ArrowForward />Not your school?
            </a>
          </Col>
          <Col>
            {this.props.children}
          </Col>
        </Row>
        <Row>
          <Navbar bg="dark" variant="dark">
            <Navbar.Brand href="#home">
              <img
                alt="Innexgo Logo"
                src={innexgo_logo}
                width="30"
                height="30"
                className="d-inline-block align-top"
              />{' '}
              Innexgo
          </Navbar.Brand>
            <Nav>
              <Nav.Link>&copy; Innexgo LLC, 2020</Nav.Link>
              <Nav.Link href="/terms_of_service">Terms of Service</Nav.Link>
              <Nav.Link href="/terms_of_service#cookie_policy">Cookie Policy</Nav.Link>
            </Nav>
          </Navbar>
        </Row>
      </Container>
    )
  }
}

export default SimpleLayout;
