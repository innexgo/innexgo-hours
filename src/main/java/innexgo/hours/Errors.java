/*
 * Innexgo Website
 * Copyright (C) 2020 Innexgo LLC
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

package innexgo.hours;

import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;

public enum Errors {
  OK(HttpStatus.OK),
  NOT_FOUND(HttpStatus.NOT_FOUND),
  NO_CAPABILITY(HttpStatus.UNAUTHORIZED),
  API_KEY_UNAUTHORIZED(HttpStatus.UNAUTHORIZED),
  PASSWORD_INCORRECT(HttpStatus.UNAUTHORIZED),
  PASSWORD_INSECURE(HttpStatus.UNAUTHORIZED),
  PASSWORD_CANNOT_CREATE_FOR_OTHERS(HttpStatus.UNAUTHORIZED),
  USER_NONEXISTENT(HttpStatus.BAD_REQUEST),
  API_KEY_NONEXISTENT(HttpStatus.BAD_REQUEST),
  USER_EXISTENT(HttpStatus.BAD_REQUEST),
  USER_NAME_EMPTY(HttpStatus.BAD_REQUEST),
  USER_EMAIL_EMPTY(HttpStatus.BAD_REQUEST),
  USER_EMAIL_INVALIDATED(HttpStatus.BAD_REQUEST),
  USER_KIND_INVALID(HttpStatus.BAD_REQUEST),

  SCHOOL_NONEXISTENT(HttpStatus.BAD_REQUEST),


  SESSION_REQUEST_NONEXISTENT(HttpStatus.BAD_REQUEST),
  SESSION_REQUEST_RESPONSE_EXISTENT(HttpStatus.BAD_REQUEST),
  SESSION_REQUEST_RESPONSE_CANNOT_CANCEL_STUDENT(HttpStatus.BAD_REQUEST),

  SESSION_CANNOT_CREATE_FOR_OTHERS_STUDENT(HttpStatus.BAD_REQUEST),

  SESSION_NONEXISTENT(HttpStatus.BAD_REQUEST),


  COMMITTMENT_EXISTENT(HttpStatus.BAD_REQUEST),
  COMMITTMENT_NONEXISTENT(HttpStatus.BAD_REQUEST),
  COMMITTMENT_CANNOT_CREATE_FOR_OTHERS_STUDENT(HttpStatus.BAD_REQUEST),
  COMMITTMENT_CANNOT_CREATE_HIDDEN_STUDENT(HttpStatus.BAD_REQUEST),
  COMMITTMENT_CANNOT_CREATE_UNCANCELLABLE_STUDENT(HttpStatus.BAD_REQUEST),

  COMMITTMENT_RESPONSE_KIND_INVALID(HttpStatus.BAD_REQUEST),
  COMMITTMENT_RESPONSE_EXISTENT(HttpStatus.BAD_REQUEST),
  COMMITTMENT_RESPONSE_UNCANCELLABLE(HttpStatus.BAD_REQUEST),
  COMMITTMENT_RESPONSE_CANNOT_CREATE_FOR_OTHERS_STUDENT(HttpStatus.BAD_REQUEST),

  COURSE_NONEXISTENT(HttpStatus.BAD_REQUEST),

  COURSEMEMBERSHIP_NONEXISTENT(HttpStatus.BAD_REQUEST),
  COURSEMEMBERSHIP_CANNOT_REMOVE_SELF(HttpStatus.UNAUTHORIZED),

  LOCATION_NONEXISTENT(HttpStatus.BAD_REQUEST),

  NEGATIVE_DURATION(HttpStatus.BAD_REQUEST),
  CANNOT_ALTER_PAST(HttpStatus.BAD_REQUEST),


  VERIFICATION_CHALLENGE_NONEXISTENT(HttpStatus.BAD_REQUEST),
  VERIFICATION_CHALLENGE_TIMED_OUT(HttpStatus.BAD_REQUEST),
  PASSWORD_RESET_NONEXISTENT(HttpStatus.BAD_REQUEST),
  PASSWORD_EXISTENT(HttpStatus.BAD_REQUEST),
  PASSWORD_RESET_TIMED_OUT(HttpStatus.BAD_REQUEST),
  EMAIL_RATELIMIT(HttpStatus.TOO_MANY_REQUESTS),
  EMAIL_BLACKLISTED(HttpStatus.FORBIDDEN),
  UNKNOWN(HttpStatus.INTERNAL_SERVER_ERROR);

  private final HttpStatus httpStatus;

  private Errors(HttpStatus status) {
    this.httpStatus = status;
  }

  public ResponseEntity<?> getResponse() {
    return new ResponseEntity<>("\"" + this.name() + "\"", httpStatus);
  }
}
