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

import java.sql.ResultSet;
import java.sql.SQLException;
import org.springframework.jdbc.core.RowMapper;

public class PasswordRowMapper implements RowMapper<Password> {

  @Override
  public Password mapRow(ResultSet row, int rowNum) throws SQLException {
    Password ps = new Password();
    ps.passwordId= row.getLong("password_id");
    ps.creationTime = row.getLong("creation_time");
    ps.creatorUserId = row.getLong("creator_user_id");
    ps.userId = row.getLong("user_id");
    ps.passwordKind = PasswordKind.from(row.getInt("password_kind"));
    ps.passwordHash = row.getString("password_hash");
    ps.passwordResetKeyHash = row.getString("password_reset_key_hash");
    return ps;
  }
}