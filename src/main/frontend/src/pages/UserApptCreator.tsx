import React from 'react'
import { Form, Button } from 'react-bootstrap';
import DashboardLayout from '../components/DashboardLayout';

export default function MakeAppt(props: AuthenticatedComponentProps) {
  const formStyle = {
    padding: '0% 3%',
  }
  const headerStyle = {
    marginTop: '2%',
    textAlign: 'center' as const,
  }
  const buttonStyle = {
    marginTop: '2%',
  }
  return (
  <DashboardLayout name={props.apiKey.user.name} logoutCallback={()=>props.setApiKey(null)} >
      <h1 style={headerStyle}>Make an Appointment</h1>
      <Form style={formStyle}>
        <Form.Group controlId="date">
          <Form.Label>Date</Form.Label>
          <Form.Control type="date" />
        </Form.Group>
        <Form.Group controlId="time">
          <Form.Label>Current Time</Form.Label>
          <Form.Control type="time" />
        </Form.Group>

        <Form.Group controlId="teacher">
          <Form.Label>Student</Form.Label>
          <Form.Control as="select">
            <option>Marek</option>
            <option>Richard</option>
            <option>Govind</option>
          </Form.Control>
        </Form.Group>

        <Form.Group controlId="message">
          <Form.Label>Message</Form.Label>
          <Form.Control as="textarea" rows={3} />
        </Form.Group>

        <Button style={buttonStyle} variant="primary" type="submit">Submit</Button>
      </Form>
    </DashboardLayout>
  );
}
