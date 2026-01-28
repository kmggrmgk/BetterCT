import uuid
from datetime import datetime
from sqlalchemy import Column, String, Text, DateTime, Enum
import enum
from backend.app.core.database import Base

class TaskStatus(str, enum.Enum):
